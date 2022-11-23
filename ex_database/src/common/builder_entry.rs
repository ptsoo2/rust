use r2d2::{Builder, CustomizeConnection, HandleError, HandleEvent, ManageConnection};
use std::time::Duration;

pub struct Config<TStub>
where
    TStub: ManageConnection,
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

impl<TStub> Default for Config<TStub>
where
    TStub: ManageConnection,
{
    fn default() -> Self {
        Self {
            max_size: 10,
            min_idle: None,
            test_on_check_out: false,
            max_lifetime: None,
            idle_timeout: None,
            connection_timeout: Duration::from_secs(10),
            error_handler: Box::new(r2d2::LoggingErrorHandler),
            event_handler: Box::new(r2d2::NopEventHandler),
            connection_customizer: Box::new(r2d2::NopConnectionCustomizer),
        }
    }
}

pub(crate) fn make_builder<TStub>() -> Builder<TStub>
where
    TStub: ManageConnection,
{
    Builder::new()
}

pub(crate) fn make_configured_builder<TStub>(config: Config<TStub>) -> Builder<TStub>
where
    TStub: ManageConnection,
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
