use std::collections::HashSet;

use anyhow::bail;
use ex_common::{log};
use std::fs;
use crate::config_format::ServerGroup;

pub struct CConfig {
	pub server_group_: ServerGroup,
	path_: String,
}

impl Default for CConfig {
	fn default() -> Self {
		Self {
			server_group_: ServerGroup::default(),
			path_: String::new(),
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

impl CConfig {
	pub fn load(mut self, path: String, load_type: EConfigLoadType) -> anyhow::Result<CConfig> {
		self.path_ = path;
		let str_path = &self.path_[..];
		log!("config path: {}", str_path);
		
		self.server_group_ = match load_type {
			EConfigLoadType::YAML => self._load_from_yaml(str_path),
			EConfigLoadType::XML => self._load_from_xml(str_path),
			EConfigLoadType::JSON => self._load_from_json(str_path),
			_ => {
				todo!()
			}
		}?;
		
		log!("config contents: {:?}", self.server_group_);
		self._verify()?;
		Ok(self)
	}
	
	fn _verify(&self) -> anyhow::Result<()> {
		let mut port_verifier = HashSet::new();
		for _server_config in &self.server_group_.server_group {
			let port = _server_config.host.port;
			if port_verifier.insert(port) == false {
				bail!("duplicate port({})", port);
			}
		}
		Ok(())
	}
	
	fn _load_from_yaml(&self, path: &str) -> anyhow::Result<ServerGroup, anyhow::Error> {
		let str_config = fs::read_to_string(path)?;
		let a = serde_yaml::from_str::<ServerGroup>(&str_config[..]);
		if a.is_err() == true {
			bail!("{:?}", a.err());
		}
		Ok(a.unwrap())
	}
	
	fn _load_from_xml(&self, path: &str) -> anyhow::Result<ServerGroup> {
		let _str_config = fs::read_to_string(path)?;
		
		todo!()
	}
	
	fn _load_from_json(&self, path: &str) -> anyhow::Result<ServerGroup> {
		let _str_config = fs::read_to_string(path)?;
		
		todo!();
	}
}
