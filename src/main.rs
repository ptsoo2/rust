use std::env;
use extern_config::config;

mod app;

fn main() -> anyhow::Result<()> {
	init_singletons();
	
	let config_path = config::parse_config_path(env::args().collect())?;
	app::get_instance().load_config(config_path)
}

pub fn init_singletons()
{
	let _ret = app::get_instance();
}