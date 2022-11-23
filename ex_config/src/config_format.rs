use serde::{Deserialize, Serialize};

// use crate::stdafx::config_format::default::{host, naming};

/*
 * 지원하는 Element 를 적는다.
 * 순서는 상관없다.
* 이름은 정확하게 일치해야한다.  */
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Host {
    pub ip: String,
    pub port: u16,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Auth {
    pub user: String,
    pub password: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Naming {
    pub service_name: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Customize {
    pub env_type: String,
    pub worker_count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerConfig {
    pub host: Host,
    pub naming: Naming,
    pub customize: Customize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: Host {
                ip: ("127.0.0.1").to_owned(),
                port: 30002,
            },
            naming: Naming {
                service_name: ("test_service").to_owned(),
            },
            customize: Customize {
                env_type: ("debug").to_owned(),
                worker_count: (5),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerGroup {
    pub data: Vec<ServerConfig>,
}

impl Default for ServerGroup {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RedisConfig {
    pub host: Host,
    pub db_no: i64,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: Host {
                ip: ("127.0.0.1").to_owned(),
                port: 6379,
            },
            db_no: 1,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RedisGroup {
    pub data: Vec<RedisConfig>,
}

impl Default for RedisGroup {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MQPublishExchange {
    pub direct: String,
    pub fanout: String,
}

impl Default for MQPublishExchange {
    fn default() -> Self {
        Self {
            direct: String::new(),
            fanout: String::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MQConf {
    pub mem_channel_bound: usize,
    pub buffered_writes_high_water: usize,
    pub buffered_writes_low_water: usize,
    pub auth: Auth,
    pub host: Host,
    pub publish_exchange: MQPublishExchange,
}

impl Default for MQConf {
    fn default() -> Self {
        Self {
            mem_channel_bound: 16,
            buffered_writes_high_water: 16 << 20,
            buffered_writes_low_water: 0,
            auth: Auth {
                user: ("admin").to_owned(),
                password: ("admin").to_owned(),
            },
            host: Host {
                ip: ("127.0.0.1").to_owned(),
                port: 5672,
            },
            publish_exchange: MQPublishExchange::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MySQLSchemaConf {
    pub host: Host,
    pub auth: Auth,
    pub schema_name: String,
}

impl Default for MySQLSchemaConf {
    fn default() -> Self {
        Self {
            host: Host {
                ip: ("127.0.0.1").to_owned(),
                port: 13306,
            },
            auth: Auth {
                user: ("server_dev").to_owned(),
                password: ("wkddnjsdud").to_owned(),
            },
            schema_name: ("game_db").to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MySQLConf {
    pub data: Vec<MySQLSchemaConf>,
    pub max_connections: u32,
}

impl Default for MySQLConf {
    fn default() -> Self {
        Self {
            data: vec![MySQLSchemaConf::default()],
            max_connections: 1 << 5,
        }
    }
}
