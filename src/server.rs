use rocket::{Build, Rocket};

use crate::api::account_new::{account_exists, account_new, test_account_new};

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

    #[get("/echo_test")]
    pub(crate) fn echo_test() -> String {
        // todo! 상태 코드나 json 으로 떤져줄때 어떻게할지 생각해봐야함
        "👋 Hello, i'm server1!".to_string()
    }
}

mod port2 {}

pub fn mount_port1(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", routes![port1::home])
        .mount("/", routes![port1::shutdown])
        .mount("/", routes![port1::echo_test])
        .mount("/", routes![account_exists])
        .mount("/", routes![test_account_new])
        .mount("/", routes![account_new])
}

#[allow(unused)]
pub fn mount_port2(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", routes![port1::home])
        .mount("/", routes![port1::shutdown])
}
