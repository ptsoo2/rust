use rocket::{Build, Rocket};

use super::{account, nickname};

pub(crate) mod port1 {
    use rocket::{http::Status, response::status::Custom, Shutdown};

    use crate::account_server::api::send_response;

    #[get("/")]
    pub(crate) fn home() -> String {
        "👋 Hello, i'm server1!".to_string()
    }
    #[get("/shutdown")]
    pub(crate) fn shutdown(shutdown: Shutdown) -> &'static str {
        shutdown.notify();
        "Shutting down..."
    }

    #[get("/ping")]
    pub(crate) fn ping() -> Custom<String> {
        send_response(Status::Ok, Some("pong"))
    }
}

mod port2 {}

pub fn mount_port1(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket
        .mount("/", routes![port1::home])
        .mount("/", routes![port1::shutdown])
        .mount("/", routes![port1::ping]);

    let rocket = rocket
        .mount("/", routes![account::account_new])
        .mount("/", routes![account::test_account_new])
        .mount("/", routes![account::exists_account])
        .mount("/", routes![account::get_account_key]);

    rocket
        .mount("/", routes![account::get_nickname])
        .mount("/", routes![nickname::set_nickname])
        .mount("/", routes![nickname::change_nickname])
        .mount("/", routes![nickname::exists_nickname])
}

#[allow(unused)]
pub fn mount_port2(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", routes![port1::home])
        .mount("/", routes![port1::shutdown])
        .mount("/", routes![port1::ping])
}
