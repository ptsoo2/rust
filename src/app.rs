use extern_config::config;

static mut APP: Option<CApplication> = None;

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
	
	pub fn load_config(&mut self, path: String) -> anyhow::Result<()> {
		self.config_.load(path, config::EConfigLoadType::YAML)
	}
}

pub fn get_instance() -> &'static mut CApplication
{
	unsafe {
		APP.get_or_insert(CApplication::new())
	}
}
