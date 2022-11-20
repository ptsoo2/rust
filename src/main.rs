#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(associated_type_defaults)]

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
//     tests::test_thread_job_queue_performance(30, 500, 10);
// }

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?.launch().await?;

    Ok(())
}
