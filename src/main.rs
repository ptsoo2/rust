#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod server_common;
mod server;
mod command_line;

use std::thread::{sleep, Thread};
use std::time::Duration;
use amiquip::AmqpValue::{FieldTable as OtherFieldTable};
use amiquip::{Connection, ConnectionTuning, ExchangeDeclareOptions, ExchangeType, FieldTable, Publish, QueueDeclareOptions};
use ex_common::bench::bench_multiple;
use ex_common::log;
use ex_config::config::{CConfig, EConfigLoadType};
use crate::server_common::{launch_all, make_launch_hint_list};
use crate::server::mount;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
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
