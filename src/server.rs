use rocket::{Build, Rocket};

#[get("/")]
fn home_port1() -> String {
    "👋 Hello, i'm server1!".to_string()
}

#[get("/")]
fn home_port2() -> String {
    "👋 Hello, i'm server2!".to_string()
}

pub fn mount_port1(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![home_port1])
}

pub fn mount_port2(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![home_port2])
}
