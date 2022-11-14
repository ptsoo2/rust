use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt};

use anyhow::bail;
use ex_common::log;
use std::fs;

use crate::config_format;

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub server_group: config_format::ServerGroup,
    pub redis_conf: config_format::RedisGroup,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_group: config_format::ServerGroup::default(),
            redis_conf: config_format::RedisGroup::default(),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

pub enum EConfigLoadType {
    YAML = 0,
    XML,
    JSON,
    _MAX_,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Config")
            .field("server_group", &self.server_group)
            .field("redis_conf", &self.redis_conf)
            .finish()
    }
}

impl Config {
    pub fn create_and_load(path: String, load_type: EConfigLoadType) -> anyhow::Result<Config> {
        let str_path = &path[..];
        log!("config path: {}", str_path);

        let str_config = fs::read_to_string(path)?;
        let config = match load_type {
            EConfigLoadType::YAML => serde_yaml::from_str::<Config>(&str_config[..])?,
            EConfigLoadType::XML => todo!(),
            EConfigLoadType::JSON => todo!(),
            EConfigLoadType::_MAX_ => {
                bail!("Invalid LoadType!!!");
            }
        };

        log!("config contents: {:?}", config);
        config._verify()?;
        Ok(config)
    }

    fn _verify(&self) -> anyhow::Result<()> {
        // 포트 중복 체크
        let mut port_verifier = HashSet::new();
        for server_conf in &self.server_group.data {
            let port = server_conf.host.port;
            if port_verifier.insert(port) == false {
                bail!("duplicate port({})", port);
            }
        }

        // db 넘버 중복 체크
        let mut db_no_verifier = HashSet::new();
        for redis_conf in &self.redis_conf.data {
            let db_no = redis_conf.db_no;
            if db_no_verifier.insert(db_no) == false {
                bail!("duplicate db_no({})", db_no);
            }
        }

        Ok(())
    }
}
