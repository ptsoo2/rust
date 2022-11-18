#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(async_closure)]

#[macro_use]
extern crate rocket;
extern crate core;

mod app;
mod command_line;
mod database;
mod rabbitmq;
mod server;
mod server_common;
mod tests;

use futures::FutureExt;
use lapin::ExchangeKind;

use rabbitmq::amqp::{MQContext, MQRunnerBase};

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?;
    // app::get_instance().init()?.launch().await?;

    let _runner = MQRunnerBase::new(|| {
        async move {
            //
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

    // app::get_instance().init()?.launch().await?;
    // 해결
    //      recover 로직이 필요하다.
    //      publish 실패시 n번 retry 가 필요할거고,
    //      initilize 처리를 람다로 보관하고, 그걸 리커버리로 쓰자.

    Ok(())
}
