pub mod amqp {
    use ex_common::common::log;
    use ex_config::config_format;
    use futures::Future;

    use std::collections::BTreeMap;
    use std::error::Error;
    use std::pin::Pin;

    use anyhow::{bail, Ok};

    use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions};
    use lapin::protocol::basic::AMQPProperties;
    use lapin::types::AMQPValue::LongInt;
    use lapin::types::FieldTable;
    use lapin::{Channel, Connection, ConnectionProperties, ExchangeKind};

    type ChannelNo = u16;
    type Config = config_format::MQConf;

    pub struct MQContext {
        #[allow(unused)]
        conf_: Config,
        conn_: Connection,
        map_channel_: BTreeMap<ChannelNo, Channel>,
    }

    impl MQContext {
        pub async fn new(mq_conf: &Config) -> anyhow::Result<Self> {
            let conn = _connect(mq_conf).await?;
            Ok(Self {
                conf_: mq_conf.clone(),
                conn_: conn,
                map_channel_: BTreeMap::default(),
            })
        }

        pub fn is_connected(&self) -> bool {
            self.conn_.status().connected()
        }

        pub async fn close(&mut self) -> anyhow::Result<()> {
            assert_eq!(self.is_connected(), true);
            for (channel_no, channel) in self.map_channel_.iter() {
                log!("try close channel({})", channel_no);
                channel.close(1, "reply_text").await?;
            }

            self.map_channel_.clear();
            self.conn_.close(1, "reply_text").await?;
            Ok(())
        }

        pub async fn channel(&mut self) -> anyhow::Result<&mut Self> {
            assert_eq!(self.is_connected(), true);
            let channel = self.conn_.create_channel().await?;
            let channel_id = channel.id();

            if let Some(channel_id) = self.map_channel_.insert(channel_id, channel) {
                bail!("already exist channel({})", channel_id.id());
            }
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
            if let Some(channel) = self._get_channel(channel_no) {
                channel
                    .exchange_declare(
                        exchange_name.into(),
                        kind,
                        ExchangeDeclareOptions::default(),
                        FieldTable::default(),
                    )
                    .await?;
                log!("declare_exchange({}:{})", channel_no, exchange_name);
                return Ok(self);
            }
            bail!("not exist channel({})", channel_no);
        }

        pub async fn publish<TStr: Into<&'static str>, TBody: Into<String>>(
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

        async fn _reconnect(&mut self) -> anyhow::Result<&mut Self> {
            self.close().await?;
            self.conn_ = _connect(&self.conf_).await?;
            Ok(self)
        }

        fn _get_channel(&self, channel_no: u16) -> Option<&Channel> {
            self.map_channel_.get(&channel_no)
        }
    }

    async fn _connect(mq_conf: &Config) -> anyhow::Result<Connection, lapin::Error> {
        Connection::connect(&_into_uri(mq_conf)[..], _into_connect_properties(mq_conf)).await
    }

    pub struct MQRunnerBase {
        context_: Option<MQContext>,
        //fn_init_: async fn(),
    }

    // type FnInit = fn(&MQContext) -> Result<MQContext, lapin::Error>;

    //  impl MQRunnerBase {
    //      async fn new(mq_conf: &Config, fn_init: FnInit) {
    //         async move || ->anyhow::Result<()>{
    //             let a = MQContext::new(&mq_conf).await?;
    //         }
    //      }
    //             Wrapper::new(|mq_conf: &config_format::MQConf| {
    //                 Box::pin(async {
    //                     let conf = mq_conf.clone();
    //                     let a = MQContext::new(&conf).await?;
    //                     Ok(a)
    //                 })
    //             });
    //         Self { context_: None }
    //}
    //         let wrapped_init: Result<MQContext, lapin::Error> =
    //             async move || -> Result<MQContext, lapin::Error> {
    //                 let context = MQContext::new(&mq_conf).await?;
    //                 let a = fn_init(&context)?;
    //                 Ok(a)
    //             };

    //         Self { context_: None }
    //     }

    //     fn _recover(&mut self) {}
    //}

    fn _into_uri(mq_conf: &Config) -> String {
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
