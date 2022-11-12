#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod server_common;
mod server;
mod command_line;

use std::time::Duration;
use ex_config::config::{CConfig, EConfigLoadType};
use redis::{Cmd, ConnectionLike, Pipeline, };
use ex_database::redis_entry;

use crate::server_common::{launch_all, make_launch_hint_list};
use crate::server::mount;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
	
	// redis test
	{
		let connection_info = redis_entry::make_connection_info(
			"localhost",
			6379,
			1,
			None, None
		);
		
		let pool_config = redis_entry::StubConfig {
			max_size: 10,
			min_idle: None,
			test_on_check_out: false,
			max_lifetime: None,
			idle_timeout: None,
			connection_timeout: Duration::from_secs(10),
			error_handler: Box::new(r2d2::LoggingErrorHandler),
			event_handler: Box::new(r2d2::NopEventHandler),
			connection_customizer: Box::new(r2d2::NopConnectionCustomizer),
		};
		
		let pool = redis_entry::make_pool_default(
			connection_info,
			pool_config,
			None
		)?;
		
		let mut conn = pool.get()?;
		{
			let rpy = conn.req_command(Cmd::new().arg("PING"));
			println!("{:?}", rpy);
		}
		{
			// 와.. 너무 쓰레기같이 써야하네..
			let (rpy1, rpy2, rpy3): (String, String, String) = Pipeline::with_capacity(3)
				.cmd("PING")
				.cmd("SET").arg("11111111111111").arg("222222222222")
				.cmd("GET").arg("11111111111111")
				.query(&mut conn)?;
			
			println!("{}", rpy1);
			println!("{}", rpy2);
			println!("{}", rpy3);
		}
	}
	
	let command_line = command_line::CommandLine::default()
		.load()?;
	
	// load config
	let config = CConfig::default()
		.load(command_line.config_file_path_, EConfigLoadType::YAML)?;
	
	let launch_hint_list = make_launch_hint_list(
		&config.server_group_.server_group,
		&[mount, mount]
	)?;
	
	// blocking launch
	let _result = launch_all(launch_hint_list).await?;
	
	Ok(())
}
