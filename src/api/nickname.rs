use rocket::{http::Status, response::status::Custom};

use crate::db_request;

use super::{
    res::{AccountKey, ResExists, NONE_BODY},
    send_response,
};

#[put("/set_nickname/<account_key>/<nickname>")]
pub async fn set_nickname(account_key: AccountKey, nickname: String) -> Custom<String> {
    // todo! account_id validate
    if let Ok(_) = db_request::nickname::set_nickname(account_key, nickname).await {
        return send_response(Status::Ok, NONE_BODY);
    }

    send_response(Status::InternalServerError, NONE_BODY)
}

#[patch("/change_nickname/<account_key>/<nickname>")]
pub async fn change_nickname(account_key: AccountKey, nickname: String) -> Custom<String> {
    // todo! account_id validate
    if let Ok(_) = db_request::nickname::change_nickname(account_key, nickname).await {
        return send_response(Status::Ok, NONE_BODY);
    }

    send_response(Status::InternalServerError, NONE_BODY)
}

#[patch("/exists_nickname/<nickname>")]
pub async fn exists_nickname(nickname: String) -> Custom<String> {
    // todo! account_id validate
    if let Ok(_) = db_request::nickname::get_account_key_with_nickname(nickname).await {
        let res = ResExists { is_exist: true };
        return send_response(Status::Ok, Some(res));
    }

    send_response(Status::InternalServerError, NONE_BODY)
}
