#![feature(decl_macro)]

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

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?.launch().await?;

    // 으 개오바다...
    // 부하 원인은 exchange 를 매번 만들어서이다.
    // 라이브러리가 자기 참조 형태로 작동하는데 이걸 어떻게 해봐야하겠다.
    // app::get_instance().init()?;
    // test_mq_no_context(); // 324.1261ms
    // test_mq_publish(); // 9.500251s

    Ok(())
}
