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
pub struct Redis {
    pub host: Host,
    pub db_no: i64,
}

impl Default for Redis {
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
