use std::{error, fmt};
use std::error::Error as _StdError;
use std::sync::Arc;
use std::time::Duration;
use r2d2::{Builder, CustomizeConnection, HandleError, HandleEvent, ManageConnection, Pool};
use redis::{Cmd, Connection, ConnectionInfo, ConnectionLike, IntoConnectionInfo, RedisConnectionInfo, RedisError};
use redis::ConnectionAddr::Tcp;

pub fn make_connection_info(ip: &str, port: u16, dbNo: i64, username: Option<&str>, password: Option<&str>) -> ConnectionInfo {
	ConnectionInfo {
		addr: Tcp(ip.to_owned(), port),
		redis: RedisConnectionInfo {
			db: dbNo,
			username: if let Some(username) = username { Some(username.to_owned()) } else { None },
			password: if let Some(password) = password { Some(password.to_owned()) } else { None },
		},
	}
}

pub struct Config<C, E> {
	pub max_size: u32,
	pub min_idle: Option<u32>,
	pub test_on_check_out: bool,
	pub max_lifetime: Option<Duration>,
	pub idle_timeout: Option<Duration>,
	pub connection_timeout: Duration,
	// pub error_handler: Box<dyn HandleError<E>>,
	// pub event_handler: Box<dyn HandleEvent>,
	pub connection_customizer: Box<dyn CustomizeConnection<C, E>>,
}

pub struct Stub {
	connection_info_: redis::ConnectionInfo,
}

impl r2d2::ManageConnection for Stub {
	type Connection = redis::Connection;
	type Error = RedisError;
	
	fn connect(&self) -> anyhow::Result<Connection, Self::Error> {
		let client = redis::Client::open(self.connection_info_.clone())?;
		client.get_connection()
	}
	
	fn is_valid(&self, conn: &mut Self::Connection) -> anyhow::Result<(), Self::Error> {
		let _ = conn.req_command(Cmd::new().arg("PING"))?;
		Ok(())
	}
	
	fn has_broken(&self, conn: &mut Self::Connection) -> bool {
		conn.is_open() == false
	}
}

type builder_t = Builder<Stub>;
type pool_t = Pool<Stub>;
pub type pool_config_t = Config<Stub, RedisError>;
type fn_build_hook_t = Option<fn(&mut builder_t)>;

#[derive(Debug)]
pub struct NamespaceCustomizer {}

impl CustomizeConnection<Stub, RedisError> for NamespaceCustomizer {
	fn on_acquire(&self, stub: &mut Stub) -> Result<(), RedisError> {
		Ok(())
	}
}

pub fn make_pool_default(
	connnection_info: ConnectionInfo,
	config: pool_config_t,
	fn_build_hook: fn_build_hook_t
) -> anyhow::Result<pool_t> {
	let fn_configured_pool = || {
		let builder = Builder::new()
			.max_size(config.max_size);
		builder
	};
	
	let mut builder = fn_configured_pool();
	if fn_build_hook.is_none() == false {
		fn_build_hook.unwrap()(&mut builder);
	}
	
	let stub = Stub { connection_info_: connnection_info };
	let pool = builder.build(stub)?;
	Ok(pool)
}