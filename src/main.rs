#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(associated_type_defaults)]

use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};

#[allow(unused)]
use tests::{mq, redis, starter, thread as test_thread};
use third_party::EMySQLType;

#[macro_use]
extern crate rocket;
extern crate core;

mod app;
mod command_line;
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
    tokio::spawn(async move {
        thread::sleep(Duration::from_secs(5));
        async fn test() -> anyhow::Result<()> {
            let pool = app::get_instance()
                .get_mysql_pool(EMySQLType::ACCOUNT)
                .unwrap();

            let guild = sqlx::query_as::<_, Subal>("SELECT now() as now")
                .fetch_one(pool)
                .await?;

            println!("{:?}", guild);
            Ok(())
        }

        if let Err(e) = test().await {
            println!("{:?}", e);
        }
    });

    app::get_instance().init()?.launch().await?;

    Ok(())
}
