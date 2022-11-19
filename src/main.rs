#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(async_closure)]

#[macro_use]
extern crate rocket;
extern crate core;

mod app;
mod command_line;
mod database;
mod server;
mod server_common;
mod tests;

use ex_rabbitmq::{context::MQContext, runner::MQRunnerBase};
use futures::FutureExt;
use lapin::ExchangeKind;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?;
    // app::get_instance().init()?.launch().await?;

    let _runner = MQRunnerBase::new(|| {
        async move {
            let mq_conf = &app::get_instance().get_config().mq_conf;
            let mut context = MQContext::new(mq_conf).await?;
            context
                .channel()
                .await?
                .declare_exchange(1, "game_server.direct", ExchangeKind::Direct)
                .await?;
            Ok(context)
        }
        .boxed()
    });

    Ok(())
}
