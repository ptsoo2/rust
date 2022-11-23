use rocket::{Build, Rocket};

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
    rocket
        .mount("/", routes![port1::home])
        .mount("/", routes![port1::shutdown])
}

#[allow(unused)]
pub fn mount_port2(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", routes![port1::home])
        .mount("/", routes![port1::shutdown])
}
