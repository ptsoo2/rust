#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(associated_type_defaults)]

use std::{thread, time::Duration};

use ex_rabbitmq::message::Message;
use lapin::options::BasicPublishOptions;

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
    app::get_instance().init()?;

    let jh = thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let a = app::get_instance().get_mq_publisher();
        loop {
            a.publish(Message {
                app_id_: "123123".to_owned(),
                body_: "hello world!!!!!!!!!!!!".to_owned(),
                exchange_: "game_server.direct".to_owned(),
                routing_key_: "1123123".to_owned(),
                channel_no_: 1,
                basic_publish_options_: BasicPublishOptions {
                    mandatory: false,
                    immediate: false,
                },
            });
        }
    });

    app::get_instance().launch().await?;
    jh.join().expect("!!!");

    Ok(())
}
