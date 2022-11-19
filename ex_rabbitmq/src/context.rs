use std::collections::BTreeMap;

use anyhow::bail;

use ex_common::common::log;
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions};
use lapin::protocol::basic::AMQPProperties;
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ExchangeKind};

use crate::common::_make_connection;

type Config = ex_config::config_format::MQConf;
type ChannelNo = u16;

pub struct MQContext {
    #[allow(unused)]
    conf_: Config,
    conn_: Connection,
    map_channel_: BTreeMap<ChannelNo, Channel>,
}

impl MQContext {
    pub async fn new(mq_conf: &Config) -> anyhow::Result<Self> {
        let conn = _make_connection(mq_conf).await?;
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
        self.conn_ = _make_connection(&self.conf_).await?;
        Ok(self)
    }

    fn _get_channel(&self, channel_no: u16) -> Option<&Channel> {
        self.map_channel_.get(&channel_no)
    }
}
