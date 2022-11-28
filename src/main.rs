#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(associated_type_defaults)]

use serde::{Deserialize, Serialize};

#[allow(unused)]
use tests::{mq, redis, starter, thread as test_thread};

#[macro_use]
extern crate rocket;
extern crate core;

mod api;
mod app;
mod command_line;
mod db_request;
mod server;
mod server_common;
mod tests;
mod third_party;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subal {
    pub now: chrono::DateTime<chrono::Utc>,
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?.launch().await?;

    Ok(())
}
