use rocket::{http::Status, response::status::Custom};

use super::{
    res::{AccountKey, ResAccountKey, ResExists, ResNickname, INVALID_ACCOUNT_KEY, NONE_BODY},
    send_response,
};
use crate::account_server::db_request;

#[get("/exists_account/<account_id>")]
pub async fn exists_account(account_id: String) -> Custom<String> {
    // todo! account_id validate
    if let Ok(account_key) = db_request::account_key::get_account_key(account_id).await {
        let mut res = ResExists { is_exist: false };
        if account_key != INVALID_ACCOUNT_KEY {
            res.is_exist = true;
        }

        return send_response(Status::Ok, Some(res));
    }

    send_response(Status::InternalServerError, NONE_BODY)
}
