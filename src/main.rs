use std::net::{Ipv4Addr};

use ex_net::common::get_my_ip;

mod app;
mod tests;

fn main() -> anyhow::Result<()> {
	init_singletons();
	
	app::get_instance().start()?;
	
	let host = &app::get_instance().get_config_data().host;
	ex_net::listener::startup(&host.ip, host.port)?;
	Ok(())
}

pub fn init_singletons() {
	let _ret = app::get_instance();
}