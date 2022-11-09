use rocket::{Build, Rocket};

#[get("/index", format = "json")]
fn index() -> &'static str {
	"{\"version\": \"1.11.1\"}"
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
	rocket.mount("/", routes![index])
}
