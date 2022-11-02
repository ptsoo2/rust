use std::env;
use chrono::Local;
use ex_common::{
	log, function
};
use ex_config::config;
use ex_config::config_format::ConfigGroup;

static mut APP: Option<CApplication> = None;

pub struct CApplication {
	pub command_args_: Vec<String>,
	config_: config::CConfig,
}

impl Default for CApplication {
	fn default() -> Self {
		Self {
			command_args_: Vec::default(),
			config_: config::CConfig::default(),
		}
	}
}

impl CApplication {
	pub fn new() -> Self {
		Self { ..Self::default() }
	}
	
	fn _load_config(&mut self) -> anyhow::Result<()> {
		// save commandArgs
		self.command_args_ = env::args().collect();
		log!("commmands: {:?}", self.command_args_);
		
		// load
		self.config_.load(
			config::parse_config_path(&self.command_args_)?,
			config::EConfigLoadType::YAML
		)
	}
	
	fn _init(&mut self) -> anyhow::Result<()> {
		self._load_config()
	}
	
	pub fn get_config_data(&self) -> &ConfigGroup {
		&self.config_.data_
	}
	
	pub fn start(&mut self) -> anyhow::Result<()> {
		self._init()?;
		Ok(())
	}
}

pub fn get_instance() -> &'static mut CApplication
{
	unsafe {
		APP.get_or_insert(CApplication::new())
	}
}
