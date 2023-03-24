use ex_common::log;
use rocket::{
    http::Status,
    response::status::{self, Custom},
};
use serde::Serialize;

pub mod account;
pub mod nickname;
pub mod res;
pub mod server;

pub(crate) fn make_common_body() -> String {
    "{\"err\": make_common_body}".to_string()
}

pub(crate) fn send_response<T>(status: Status, body: Option<T>) -> Custom<String>
where
    T: Serialize,
{
    match body {
        Some(body) => match serde_json::to_string(&body) {
            Ok(body) => status::Custom(status, body),
            Err(_) => {
                log!("failed json to_string");
                status::Custom(Status::InternalServerError, make_common_body())
            }
        },
        None => status::Custom(status, make_common_body()),
    }
}
