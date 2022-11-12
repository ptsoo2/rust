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
use r2d2::{Builder, Pool, PooledConnection, };

use redis::{ConnectionInfo, ConnectionLike, IntoConnectionInfo, RedisConnectionInfo, RedisError};
use redis::ConnectionAddr::Tcp;
use rocket::Config;

use ex_database::redis_entry;

use ex_database::redis_entry::{Stub};
use crate::server_common::{launch_all, make_launch_hint_list};
use crate::server::mount;

fn do_something(conn: &mut redis::Connection) -> redis::RedisResult<()> {
	redis::cmd("SET").arg("1231231231").arg("!23121231212312312123").query(conn)?;
	Ok(())
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
	let connection_info =
		redis_entry::make_connection_info("localhost", 6379, 1, None, None);
	
	let pool_config = redis_entry::pool_config_t {
		max_size: 10,
		min_idle: None,
		test_on_check_out: false,
		max_lifetime: None,
		idle_timeout: None,
		connection_timeout: Default::default(),
		connection_customizer: Box::new(redis_entry::NamespaceCustomizer {}),
	};
	
	let pool = redis_entry::make_pool_default(
		connection_info,
		pool_config,
		None)?;
	
	let conn = pool.get()?;
	
	//let connection = a.get()?;
	
	// let conn_manager: ConnectionManagerStub = ConnectionManagerStub::new(connection_info)?;
	// let a = r2d2::Pool::builder().build(conn_manager)?;
	
	// let client = redis::Client::open(connection_info)?;
	
	// let mut client: redis::Connection = client.get_connection()?;
	// println!("{}", client.get_db());
	//
	// do_something(&mut client)?;
	//
	//let a = RedisConnectionManager::new(connection_info);
	
	// let command_line = command_line::CommandLine::default()
	// 	.load()?;
	//
	// // load config
	// let config = CConfig::default()
	// 	.load(command_line.config_file_path_, EConfigLoadType::YAML)?;
	//
	// let launch_hint_list = make_launch_hint_list(
	// 	&config.server_group_.server_group,
	// 	&[mount, mount]
	// )?;
	//
	// // blocking launch
	// let _result = launch_all(launch_hint_list).await?;
	
	Ok(())
}
