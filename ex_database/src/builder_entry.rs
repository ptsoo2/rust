use std::time::Duration;
use r2d2::{
	Builder,
	CustomizeConnection,
	ManageConnection,
	HandleError,
	HandleEvent
};

pub struct Config<TStub>
	where TStub: ManageConnection
{
	pub max_size: u32,
	pub min_idle: Option<u32>,
	pub test_on_check_out: bool,
	pub max_lifetime: Option<Duration>,
	pub idle_timeout: Option<Duration>,
	pub connection_timeout: Duration,
	pub error_handler: Box<dyn HandleError<TStub::Error>>,
	pub event_handler: Box<dyn HandleEvent>,
	pub connection_customizer: Box<dyn CustomizeConnection<TStub::Connection, TStub::Error>>,
}

pub(crate) fn make_builder<TStub>() -> Builder<TStub>
	where TStub: ManageConnection
{
	Builder::new()
}

pub(crate) fn make_configured_builder<TStub>(config: Config<TStub>) -> Builder<TStub>
                                             where TStub: ManageConnection
{
	make_builder()
		.max_size(config.max_size)
		.min_idle(config.min_idle)
		.test_on_check_out(config.test_on_check_out)
		.max_lifetime(config.max_lifetime)
		.idle_timeout(config.idle_timeout)
		.connection_timeout(config.connection_timeout)
		.connection_customizer(config.connection_customizer)
}
