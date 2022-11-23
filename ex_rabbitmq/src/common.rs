use lapin::types::AMQPValue::LongInt;
use lapin::types::FieldTable;
use lapin::{Connection, ConnectionProperties};

type Config = ex_config::config_format::MQConf;

pub(crate) async fn _make_connection(mq_conf: &Config) -> anyhow::Result<Connection, lapin::Error> {
    Connection::connect(&_into_uri(mq_conf)[..], _into_connect_properties(mq_conf)).await
}

pub(crate) fn _into_uri(mq_conf: &Config) -> String {
    ("amqp://").to_owned()
        + &mq_conf.auth.user
        + ":"
        + &mq_conf.auth.password
        + "@"
        + &mq_conf.host.ip
        + ":"
        + &mq_conf.host.port.to_string()
}

pub(crate) fn _into_connect_properties(_conf: &Config) -> ConnectionProperties {
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
