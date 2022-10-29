use std::fs;

use extern_common::common;
use crate::config_format;
use config_format::ConfigGroup;

pub fn parse_config_path(args: Vec<String>) -> anyhow::Result<String> {
	let config_path: String = match args.len() < 2 {
		true => {
			// 없는 경우 하드코딩
			common::get_current_path_str() + "/cfg/config.yaml"
		}
		
		// 있는 경우 1번째 인자를 패스로
		false => args.get(1).unwrap().to_owned(),
	};
	Ok(config_path)
}

pub struct CConfig {
	path_: String,
	data_: ConfigGroup,
}

impl Default for CConfig {
	fn default() -> Self {
		Self {
			path_: ("../cfg/config2.yaml").to_owned(),
			data_: ConfigGroup::default(),
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
	pub fn load(&mut self, path: String, load_type: EConfigLoadType) -> anyhow::Result<()> {
		self.path_ = path;
		let str_path = &self.path_[..];
		println!("Config path: {}", str_path);
		
		self.data_ = match load_type {
			EConfigLoadType::YAML => self._load_from_yaml(str_path),
			EConfigLoadType::XML => self._load_from_xml(str_path),
			EConfigLoadType::JSON => self._load_from_json(str_path),
			_ => {
				todo!()
			}
		}?;
		
		println!("{:?}", self.data_);
		Ok(())
	}
	
	fn _load_from_yaml(&self, path: &str) -> anyhow::Result<ConfigGroup> {
		let str_config = fs::read_to_string(path)?;
		let ret = serde_yaml::from_str::<ConfigGroup>(&str_config[..]);
		
		Ok(ret.unwrap())
	}
	
	fn _load_from_xml(&self, path: &str) -> anyhow::Result<ConfigGroup> {
		let _str_config = fs::read_to_string(path)?;
		
		todo!()
	}
	
	fn _load_from_json(&self, path: &str) -> anyhow::Result<ConfigGroup> {
		let _str_config = fs::read_to_string(path)?;
		
		todo!();
	}
}
