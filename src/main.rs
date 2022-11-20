#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(associated_type_defaults)]

use std::{thread, time::Duration};

use ex_common::{common::get_tid, log};

#[macro_use]
extern crate rocket;
extern crate core;

mod app;
mod command_line;
mod server;
mod server_common;
mod tests;
mod third_party;

// test-boundary
// fn main() {
//     // tests::test_thread_job_queue_performance(30, 500, 10);
// }

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    log!("{:?}", get_tid());

    app::get_instance().init()?;

    let jh = thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let a = app::get_instance().get_mq_publisher();
        loop {
            a.publish("123123132132123".to_string());
        }
    });

    app::get_instance().launch().await?;
    jh.join().expect("!!!");

    Ok(())
}
