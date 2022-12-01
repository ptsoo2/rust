use rocket::{Build, Rocket};

use crate::api;

pub(crate) mod port1 {
    use rocket::Shutdown;

    #[get("/")]
    pub(crate) fn home() -> String {
        "👋 Hello, i'm server1!".to_string()
    }
    #[get("/shutdown")]
    pub(crate) fn shutdown(shutdown: Shutdown) -> &'static str {
        shutdown.notify();
        "Shutting down..."
    }
}

mod port2 {}

pub fn mount_port1(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket
        .mount("/", routes![port1::home])
        .mount("/", routes![port1::shutdown]);

    let rocket = rocket
        .mount("/", routes![api::account::account_new])
        .mount("/", routes![api::account::test_account_new])
        .mount("/", routes![api::account::exists_account])
        .mount("/", routes![api::account::get_account_key]);

    rocket
        .mount("/", routes![api::account::get_nickname])
        .mount("/", routes![api::nickname::set_nickname])
        .mount("/", routes![api::nickname::change_nickname])
        .mount("/", routes![api::nickname::exists_nickname])
}

#[allow(unused)]
pub fn mount_port2(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", routes![port1::home])
        .mount("/", routes![port1::shutdown])
}
