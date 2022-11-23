#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(associated_type_defaults)]

#[allow(unused)]
use tests::{mq, redis, starter, thread as test_thread};

#[macro_use]
extern crate rocket;
extern crate core;

mod app;
mod command_line;
mod server;
mod server_common;
mod tests;
mod third_party;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?.launch().await?;
    Ok(())
}
