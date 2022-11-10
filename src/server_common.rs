use std::net::IpAddr;
use std::str::FromStr;

use anyhow::bail;

use ex_common::{
	log
};

use ex_config::config_format::{ServerConfig, };

use ex_net::common::{is_available_port};

use rocket::{Build, Config, Ignite, Rocket};

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

pub fn make_launch_hint(server_config: ServerConfig, fn_mount: fn(Rocket<Build>) -> Rocket<Build>) -> LaunchHint {
	LaunchHint {
		server_config_: server_config,
		mount_: fn_mount,
	}
}

pub fn make_launch_hint_list(server_config_list: &Vec<ServerConfig>, fn_mount_list: &[fn(Rocket<Build>) -> Rocket<Build>]) -> anyhow::Result<Vec<LaunchHint>> {
	let config_size = server_config_list.len();
	let mount_size = fn_mount_list.len();
	if (config_size == 0) || (mount_size == 0) || (config_size != mount_size) {
		bail!("mismatch size(config:{} != mount:{})", config_size, mount_size);
	}
	
	let mut ret = Vec::with_capacity(config_size);
	
	for idx in 0..config_size {
		let hint = make_launch_hint(
			server_config_list.get(idx).unwrap().clone(),
			fn_mount_list.get(idx).unwrap().clone()
		);
		
		ret.push(hint);
	}
	Ok(ret)
}

pub async fn launch_all(launch_hint_list: Vec<LaunchHint>) -> anyhow::Result<Vec<Rocket<Ignite>>> {
	let mut handle_list = Vec::with_capacity(launch_hint_list.len());
	
	for hint in &launch_hint_list {
		let rocket = _make_rocket(&hint).await?;
		let join_handle = tokio::spawn(async move {
			rocket.launch().await
		});
		
		handle_list.push(join_handle);
	}
	
	log!("all rockets are launched... blocking on({})", handle_list.len());
	
	let mut ret_rocket_list = Vec::with_capacity(launch_hint_list.len());
	
	for handle in handle_list.into_iter() {
		match handle.await? {
			Err(e) => { bail!("join error({})", e); }
			Ok(rocket) => { ret_rocket_list.push(rocket); }
		}
	}
	
	Ok(ret_rocket_list)
}

async fn _make_rocket(hint: &LaunchHint) -> anyhow::Result<Rocket<Ignite>> {
	let config = _make_config(&hint.server_config_)?;
	// config.shutdown.ctrlc = false;
	
	let rocket = Rocket::custom(config);
	let rocket = (hint.mount_)(rocket);
	_into_ignite(rocket).await
}

async fn _into_ignite(rocket: Rocket<Build>) -> anyhow::Result<Rocket<Ignite>> {
	let rocket = rocket.ignite().await?;
	let conf = rocket.config();
	match is_available_port(&conf.address, conf.port)
	{
		false => { bail!("not available port({})", conf.port); }
		true => { Ok(rocket) }
	}
}

fn _make_default_config(rhs: &ServerConfig) -> anyhow::Result<Config> {
	// todo! enum 으로 만들자
	let env_type = &rhs.customize.env_type;
	
	if env_type.eq(&("debug").to_owned()) {
		return Ok(Config::debug_default());
	} else if env_type.eq(&("release").to_owned()) {
		return Ok(Config::release_default());
	}
	
	bail!("invalid env_type({})", env_type);
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
