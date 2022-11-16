pub mod amqp {
    use ex_common::common::log;
    use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions};
    use lapin::protocol::basic::AMQPProperties;

    use std::collections::BTreeMap;

    use anyhow::bail;
    use ex_common::{get_mut_ref_member, get_ref_member};
    use ex_config::config_format;

    use lapin::types::AMQPValue::LongInt;

    use lapin::types::FieldTable;
    use lapin::{Channel, Connection, ConnectionProperties, ConnectionStatus, ExchangeKind};

    type ChannelNo = u16;
    type Config = config_format::MQConf;

    struct InnerContext {
        conn_: Connection,
        map_channel_: BTreeMap<ChannelNo, Channel>,
    }

    impl InnerContext {
        fn new(conn: Connection) -> Self {
            Self {
                conn_: conn,
                map_channel_: BTreeMap::default(),
            }
        }

        fn status(&self) -> &ConnectionStatus {
            self.conn_.status()
        }

        async fn channel(&mut self) -> anyhow::Result<ChannelNo> {
            let channel = self.conn_.create_channel().await?;
            let channel_id = channel.id();

            if let Some(v) = self.map_channel_.insert(channel_id, channel) {
                bail!("already exist channel({})", v.id());
            }

            Ok(channel_id)
        }

        async fn close(&mut self) -> anyhow::Result<(), lapin::Error> {
            for (channel_no, channel) in self.map_channel_.iter() {
                log!("try close channel({})", channel_no);
                channel.close(1, "reply_text").await?;
            }

            self.map_channel_.clear();
            self.conn_.close(1, "reply_text").await
        }

        async fn declare_exchange<TStr: Into<&'static str>>(
            &mut self,
            channel_no: u16,
            exchange_name: TStr,
            kind: ExchangeKind,
        ) -> anyhow::Result<()> {
            if let Some(channel) = self._get_channel(channel_no) {
                channel
                    .exchange_declare(
                        exchange_name.into(),
                        kind,
                        ExchangeDeclareOptions::default(),
                        FieldTable::default(),
                    )
                    .await?;
                return Ok(());
            }

            bail!("not exist channel({})", channel_no);
        }

        async fn publish<TStr: Into<&'static str>, TBody: Into<String>>(
            &self,
            channel_no: u16,
            exchange_name: TStr,
            routing_key: TStr,
            body: TBody,
        ) -> anyhow::Result<()> {
            if let Some(channel) = self._get_channel(channel_no) {
                let body: String = body.into();
                channel
                    .basic_publish(
                        exchange_name.into(),
                        routing_key.into(),
                        BasicPublishOptions::default(),
                        body.as_bytes(),
                        AMQPProperties::default(),
                    )
                    .await?;
            };
            bail!("failed to publish!!!!!!");
        }

        fn _get_channel(&self, channel_no: u16) -> Option<&Channel> {
            self.map_channel_.get(&channel_no)
        }
    }

    pub struct MQContext {
        conf_: Config,
        inner_context_: Option<InnerContext>,
    }

    impl Default for MQContext {
        fn default() -> Self {
            Self {
                conf_: config_format::MQConf::default(),
                inner_context_: None,
            }
        }
    }

    impl MQContext {
        pub fn new(mq_conf: &config_format::MQConf) -> anyhow::Result<Self> {
            Ok(Self {
                conf_: mq_conf.clone(),
                inner_context_: None,
            })
        }

        pub fn is_connected(&self) -> bool {
            match &self.inner_context_ {
                None => false,
                Some(context) => context.status().connected(),
            }
        }

        pub async fn close(&mut self) -> anyhow::Result<()> {
            assert_eq!(self.is_connected(), true);
            get_mut_ref_member!(self, inner_context_).close().await?;
            self.inner_context_ = None;
            Ok(())
        }

        pub async fn connect(&mut self) -> anyhow::Result<&mut Self> {
            assert_eq!(self.is_connected(), false);
            let conf = &self.conf_;
            let inner_context = InnerContext::new(
                Connection::connect(&_into_uri(conf)[..], _into_connect_properties(conf)).await?,
            );

            self.inner_context_ = Some(inner_context);
            Ok(self)
        }

        #[allow(unused)]
        pub async fn reconnect(&mut self) -> anyhow::Result<&mut Self> {
            self.connect().await
        }

        pub async fn channel(&mut self) -> anyhow::Result<&mut Self> {
            assert_eq!(self.is_connected(), true);
            let channel_id = get_mut_ref_member!(self, inner_context_).channel().await?;
            log!("create_channel({})", channel_id);
            Ok(self)
        }

        pub async fn declare_exchange<TStr: Into<&'static str>>(
            &mut self,
            channel_no: ChannelNo,
            exchange_name: TStr,
            kind: ExchangeKind,
        ) -> anyhow::Result<&mut Self> {
            assert_eq!(self.is_connected(), true);
            let exchange_name = exchange_name.into();

            get_mut_ref_member!(self, inner_context_)
                .declare_exchange(channel_no, exchange_name, kind)
                .await?;
            log!("declare_exchange({}:{})", channel_no, exchange_name);
            Ok(self)
        }

        pub async fn publish<TStr: Into<&'static str>, TBody: Into<String>>(
            &self,
            channel_no: u16,
            exchange_name: TStr,
            routing_key: TStr,
            body: TBody,
        ) -> anyhow::Result<()> {
            get_ref_member!(self, inner_context_)
                .publish(channel_no, exchange_name, routing_key, body)
                .await
        }
    }

    fn _into_uri(mq_conf: &config_format::MQConf) -> String {
        ("amqp://").to_owned()
            + &mq_conf.user
            + ":"
            + &mq_conf.password
            + "@"
            + &mq_conf.host.ip
            + ":"
            + &mq_conf.host.port.to_string()
    }

    fn _into_connect_properties(_conf: &Config) -> ConnectionProperties {
        let mut props = ConnectionProperties::default();
        props.locale = "ko_KR".to_owned();
        let mut field_table = FieldTable::default();
        {
            field_table.insert("max_channel".into(), LongInt(2047));
            field_table.insert("frame_size".into(), LongInt(131072));
            field_table.insert("heart_beat".into(), LongInt(30));
            props.client_properties = field_table;
        }
        props
    }
}
