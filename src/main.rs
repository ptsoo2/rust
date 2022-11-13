#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate core;

mod command_line;
mod database;
mod server;
mod server_common;

use command_line::CommandLine;
use database::boot_redis;
use ex_config::config;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    // parse commandLine
    let command_line = CommandLine::default().load()?;

    // load config
    let config = config::Config::default().load(
        command_line.config_file_path_,
        config::EConfigLoadType::YAML,
    )?;

    // process - database
    boot_redis(&config)?;

    // process - web server
    let launch_hint_list = server_common::make_launch_hint_list(
        &config.server_group.data,
        &[server::mount_port1, server::mount_port2],
    )?;

    let _result = server_common::launch_all(launch_hint_list).await?;

    Ok(())
}
