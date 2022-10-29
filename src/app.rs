use extern_config::config;
use extern_config::config_format;

pub struct CApplication {
	config_: config::CConfig,
}

impl Default for CApplication {
	fn default() -> Self {
		Self {
			config_: config::CConfig::default(),
			// data_: BTreeMap::default()
		}
	}
}

impl CApplication {
	pub fn new() -> Self {
		Self { ..Self::default() }
	}
	
	pub fn load_config(&mut self, path: String) -> bool {
		let ret = self.config_.load(path, config::eConfig_Load_Type::YAML);
		if ret.is_err() == true {
			println!("{:?}", ret.err());
			return false;
		}
		
		return true;
	}
}
