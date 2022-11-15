use amiquip::{
    AmqpProperties, Channel, Connection, ConnectionTuning, Exchange, ExchangeType, Publish,
};
use anyhow::{bail, Ok};
use ex_common::{get_ref_member, log};
use ex_config::config_format;

type CoreContext = (Connection, Channel);

pub struct MQContext {
    config_: Option<config_format::MQConf>,
    app_id_: String,
    core_: Option<CoreContext>,
}

impl MQContext {
    pub fn new(
        mq_conf: &config_format::MQConf,
        app_id: String,
        is_secure: bool,
    ) -> anyhow::Result<Self> {
        let (conn, channel) = _make_and_connect(mq_conf, is_secure)?;

        Ok(Self {
            config_: Some(mq_conf.clone()),
            app_id_: app_id,
            core_: Some((conn, channel)),
        })
    }

    #[allow(unused)]
    pub fn reconnect(&mut self, is_secure: bool) -> anyhow::Result<()> {
        self.close()?;

        let conf = get_ref_member!(self, config_);
        let (conn, channel) = _make_and_connect(conf, is_secure)?;
        self.core_ = Some((conn, channel));
        Ok(())
    }

    #[allow(unused)]
    pub fn recover(&self) -> anyhow::Result<()> {
        if self.is_connected() == false {
            bail!("not connected!!!")
        }
        let core = get_ref_member!(self, core_);
        let _ = core.1.recover(false);
        Ok(())
    }

    #[allow(unused)]
    pub fn close(&mut self) -> anyhow::Result<()> {
        self.core_ = None;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        return self.core_.is_none() == false;
    }

    // todo! - debug!!
    pub(crate) fn dbg_publish(
        &self,
        exchange_type: ExchangeType,
        routing_key: &str,
        message: &str,
    ) -> anyhow::Result<()> {
        self.publish(exchange_type, routing_key.to_owned(), message.to_owned())
    }

    pub fn publish(
        &self,
        exchange_type: ExchangeType,
        routing_key: String,
        body: String,
    ) -> anyhow::Result<()> {
        let properties = AmqpProperties::default().with_app_id(self.app_id_.clone());
        let message = Publish::with_properties(body.as_bytes(), routing_key, properties);

        let _ = self._declare_exchange(exchange_type)?.publish(message)?;
        Ok(())
    }

    fn _declare_exchange(&self, exchange_type: ExchangeType) -> anyhow::Result<Exchange> {
        if self.is_connected() == false {
            bail!("not connected!!!")
        }

        let exchange_name = self._get_declare_exchange_source(&exchange_type)?;
        let core = get_ref_member!(self, core_);

        let exchange = core.1.exchange_declare(
            // todo! // 세부 옵션도 config 로 빼자.
            exchange_type,
            &exchange_name[..],
            amiquip::ExchangeDeclareOptions {
                durable: false,
                auto_delete: false,
                internal: false,
                arguments: amiquip::FieldTable::new(),
            },
        )?;

        Ok(exchange)
    }

    fn _get_declare_exchange_source(
        &self,
        exchange_type: &ExchangeType,
    ) -> anyhow::Result<&String> {
        let conf = get_ref_member!(self, config_);

        match exchange_type {
            ExchangeType::Direct => {
                return Ok(&conf.publish_exchange.direct);
            }
            ExchangeType::Fanout => {
                return Ok(&conf.publish_exchange.fanout);
            }
            _ => {
                todo!()
            }
        }
    }
}

pub(crate) fn _make_host_key() -> String {
    ("1231231212312").to_owned()
    // todo!()
}

fn _make_and_connect(
    mq_conf: &config_format::MQConf,
    is_secure: bool,
) -> anyhow::Result<(Connection, Channel)> {
    if is_secure == true {
        todo!()
    }

    let url = _make_url(mq_conf);
    log!("mq_url: {}", url);

    let tuning = ConnectionTuning {
        mem_channel_bound: mq_conf.mem_channel_bound,
        buffered_writes_high_water: mq_conf.buffered_writes_high_water,
        buffered_writes_low_water: mq_conf.buffered_writes_low_water,
    };

    // open connection
    let mut conn = Connection::insecure_open_tuned(&url[..], tuning)?;

    // open channel
    let channel = conn.open_channel(Some(1))?;

    Ok((conn, channel))
}

fn _make_url(mq_conf: &config_format::MQConf) -> String {
    ("amqp://").to_owned()
        + &mq_conf.user
        + ":"
        + &mq_conf.password
        + "@"
        + &mq_conf.host.ip
        + ":"
        + &mq_conf.host.port.to_string()
}
