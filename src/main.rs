#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::time::Duration;
use ex_config::config::{CConfig, EConfigLoadType};

use crate::server::mount;
use crate::server_common::{launch_all, make_launch_hint};

mod tests;
mod server_common;
mod server;
mod command_line;

fn main() -> anyhow::Result<()> {
	
	// parse commandLine
	let command_line = command_line::CommandLine::default()
		.load()?;
	
	// load config
	let config = CConfig::default()
		.load(command_line.config_file_path_, EConfigLoadType::YAML)?;
	
	let launch_hint = make_launch_hint(
		&config.server_group_.server_group,
		&[mount, mount]
	)?;
	
	// launch
	launch_all(launch_hint)?;
	
	println!("run out spawn rocket");
	loop {
		std::thread::sleep(Duration::from_millis(1))
	}
}
