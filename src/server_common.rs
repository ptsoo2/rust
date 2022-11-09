use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
use std::thread;
use anyhow::__private::kind::BoxedKind;

use anyhow::bail;

use ex_config::config_format::{ServerConfig, ServerGroup};

use ex_net::common::{is_available_local_port, is_available_port};

use rocket::{Build, Config, Ignite, Rocket};
use rocket::figment::Profile;
use rocket::http::uri::fmt::UriArgumentsKind::Static;

pub struct LaunchHint {
	pub server_config_: ServerConfig,
	pub mount_: fn(Rocket<Build>) -> Rocket<Build>
}

impl Default for LaunchHint {
	fn default() -> Self {
		Self {
			server_config_: ServerConfig::default(),
			mount_: { |rocket| { rocket } }
		}
	}
}

pub fn make_launch_hint(server_config_list: &Vec<ServerConfig>, fn_mount_list: &[fn(Rocket<Build>) -> Rocket<Build>]) -> anyhow::Result<Vec<LaunchHint>> {
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

pub async fn launch_all(launch_hint_list: Vec<LaunchHint>) -> anyhow::Result<()> {
	let mut handle_list = Vec::new();
	handle_list.reserve(launch_hint_list.len());
	
	for hint in launch_hint_list {
		let rocket = _make_rocket(&hint).await?;
		handle_list.push(thread::spawn(move || {
			let e = rocket.launch();
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

pub async fn _make_rocket(hint: &LaunchHint) -> anyhow::Result<Rocket<Build>> {
	let config = _make_config(&hint.server_config_)?;
	let rocket = Rocket::custom(config);
	let rocket = (hint.mount_)(rocket);
	Ok(rocket)
	// let rocket = rocket.ignite().await?;
	// _pre_fire(rocket).await
}

async fn _pre_fire(rocket: Rocket<Ignite>) -> anyhow::Result<Rocket<Ignite>> {
	let conf = rocket.config();
	match is_available_port(&conf.address, conf.port) {
		true => Ok(rocket),
		false => { bail!("not available port({})", conf.port); }
	}
}

fn _make_default_config(rhs: &ServerConfig) -> anyhow::Result<Config> {
	// todo! enum 으로 만들자
	let debug = ("debug").to_owned();
	let release = ("release").to_owned();
	
	let env_type = &rhs.customize.env_type;
	match env_type {
		debug => { return Ok(Config::debug_default()); },
		release => { return Ok(Config::release_default()); },
		_ => { bail!("invalid env_type({})", env_type); }
	}
}

fn _make_config(rhs: &ServerConfig) -> anyhow::Result<Config> {
	let mut config = _make_default_config(rhs)?;
	
	config.address = IpAddr::from_str(&rhs.host.ip[..])?;
	config.port = rhs.host.port;
	config.workers = rhs.customize.worker_count;
	
	// trolling
	// if rhs.naming.service_name.eq("test_service2") == true {
	// 	config.address = "12312312".to_string()
	// }
	Ok(config)
}
