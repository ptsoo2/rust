use std::thread;

use anyhow::bail;

use ex_config::config_format::ServerConfig;

use ex_net::common::is_available_local_port;

use rocket::{Config, Rocket};
use rocket::config::Environment::{Development, Production, Staging};
use crate::rocket::config::{Environment};

pub struct LaunchHint {
	pub server_config_: ServerConfig,
	pub mount_: fn(Rocket) -> Rocket
}

pub fn make_launch_hint(server_config_list: &Vec<ServerConfig>, fn_mount_list: &[fn(Rocket) -> Rocket]) -> anyhow::Result<Vec<LaunchHint>> {
	let config_size = server_config_list.len();
	let mount_size = fn_mount_list.len();
	if (config_size == 0) || (mount_size == 0) || (config_size != mount_size) {
		bail!("invalid size({} != {})", config_size, mount_size);
	}
	
	let mut ret = Vec::new();
	ret.reserve(config_size);
	
	for idx in 0..config_size {
		let launch_hint = LaunchHint {
			server_config_: server_config_list.get(idx).unwrap().clone(),
			mount_: *fn_mount_list.get(idx).unwrap(),
		};
		
		ret.push(launch_hint);
	}
	Ok(ret)
}

pub fn launch_all(launch_hint_list: Vec<LaunchHint>) -> anyhow::Result<()> {
	let mut handle_list = Vec::new();
	handle_list.reserve(launch_hint_list.len());
	
	for hint in launch_hint_list {
		let rocket = _make_rocket(&hint)?;
		handle_list.push(thread::spawn(move || {
			let _e = rocket.launch();
			
			assert!(false);
		}));
	}
	
	for handle in handle_list.into_iter() {
		match handle.join() {
			Ok(_) => {},
			Err(e) => { bail!("{:?}", e); }
		}
	}
	Ok(())
}

fn _make_rocket(hint: &LaunchHint) -> anyhow::Result<Rocket> {
	let config = _make_config(&hint.server_config_)?;
	let rocket = Rocket::custom(config);
	let rocket = (hint.mount_)(rocket);
	_pre_fire(rocket)
}

fn _pre_fire(rocket: Rocket) -> anyhow::Result<Rocket> {
	let conf = rocket.config();
	match is_available_local_port(&conf.address, conf.port) {
		true => Ok(rocket),
		false => { bail!("not available port({})", conf.port); }
	}
}

fn _get_env_type(env_type: u8) -> anyhow::Result<Environment> {
	if env_type == Development as u8 {
		return Ok(Development);
	} else if env_type == Staging as u8 {
		return Ok(Staging);
	} else if env_type == Production as u8 {
		return Ok(Production);
	}
	bail!("invalid env_type({})", env_type);
}

fn _make_config(rhs: &ServerConfig) -> anyhow::Result<Config> {
	let host = &rhs.host;
	let customize = &rhs.customize;
	let environment = _get_env_type(customize.env_type)?;
	
	let config = Config::build(environment)
		.address(&host.ip)
		.port(host.port).workers(customize.worker_count)
		.finalize()?;
	
	// trolling
	// if rhs.naming.service_name.eq("test_service2") == true {
	// 	config.address = "12312312".to_string()
	// }
	Ok(config)
}
