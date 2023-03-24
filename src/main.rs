#![feature(decl_macro)]
#![feature(type_ascription)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate rocket;
extern crate core;

mod account_server;
mod app;
mod command_line;
mod server_common;
mod third_party;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    app::get_instance().init()?.launch().await?;

    Ok(())
}
