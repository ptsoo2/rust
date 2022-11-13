use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt};

use anyhow::bail;
use ex_common::log;
use std::fs;

use crate::config_format;

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub server_group: config_format::ServerGroup,
    pub redis_conf: config_format::Redis,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_group: config_format::ServerGroup::default(),
            redis_conf: config_format::Redis::default(),
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
    pub fn load(mut self, path: String, load_type: EConfigLoadType) -> anyhow::Result<Config> {
        let str_path = &path[..];
        log!("config path: {}", str_path);

        match load_type {
            EConfigLoadType::YAML => self._load_from_yaml(str_path),
            EConfigLoadType::XML => self._load_from_xml(str_path),
            EConfigLoadType::JSON => self._load_from_json(str_path),
            _ => {
                todo!()
            }
        }?;

        log!("config contents: {:?}", self.server_group);
        self._verify()?;
        Ok(self)
    }

    fn _verify(&self) -> anyhow::Result<()> {
        let mut port_verifier = HashSet::new();
        for _server_config in &self.server_group.data {
            let port = _server_config.host.port;
            if port_verifier.insert(port) == false {
                bail!("duplicate port({})", port);
            }
        }
        Ok(())
    }

    fn _load_from_yaml(&mut self, path: &str) -> anyhow::Result<()> {
        let str_config = fs::read_to_string(path)?;
        *self = serde_yaml::from_str::<Config>(&str_config[..])?;

        Ok(())
    }

    fn _load_from_xml(&mut self, path: &str) -> anyhow::Result<()> {
        let _str_config = fs::read_to_string(path)?;

        todo!()
    }

    fn _load_from_json(&mut self, path: &str) -> anyhow::Result<()> {
        let _str_config = fs::read_to_string(path)?;

        todo!();
    }
}
