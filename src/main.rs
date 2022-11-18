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

pub struct TaskQueue {}

use std::{error::Error, future::Future, pin::Pin};

use ex_common::common::print_type_of_name;
use futures::{future::BoxFuture, FutureExt};
use libc::c_void;
use rabbitmq::amqp::MQContext;

pub struct Test {
    future: Pin<Box<dyn Future<Output = bool>>>,
}

// Result<bool, Error>
fn test() -> BoxFuture<'static, Result<bool, ()>> {
    Box::pin(async { 
        Ok(true) 
    })
}

async fn test2() {
    let _a = test;

    let a = Box::pin(_a());
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?.launch().await?;
    // 해결
    //      recover 로직이 필요하다.
    //      publish 실패시 n번 retry 가 필요할거고,
    //      initilize 처리를 람다로 보관하고, 그걸 리커버리로 쓰자.

    Ok(())
}
