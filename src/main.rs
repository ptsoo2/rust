#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
extern crate core;

mod app;
mod command_line;
mod database;
mod server;
mod server_common;
mod tests;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?.launch().await?;

    Ok(())
}
