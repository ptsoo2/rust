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

#[get("/account_new/<account_id>")]
pub async fn account_new(account_id: String) -> Custom<String> {
    // todo! account_id validate
    let res = ResAccountKey {
        account_key: 123123123,
    };

    if db_request::account_key::add_account_key(account_id, res.account_key)
        .await
        .is_ok()
    {
        return send_response(Status::Ok, Some(res));
    }

    send_response(Status::InternalServerError, NONE_BODY)
}

#[get("/get_account_key/<account_id>")]
pub async fn get_account_key(account_id: String) -> Custom<String> {
    // todo! account_id validate
    if let Ok(account_key) = db_request::account_key::get_account_key(account_id).await {
        let res = ResAccountKey { account_key };
        return send_response(Status::Ok, Some(res));
    }

    send_response(Status::InternalServerError, NONE_BODY)
}

#[get("/get_nickname/<account_key>")]
pub async fn get_nickname(account_key: AccountKey) -> Custom<String> {
    // todo! account_id validate
    if let Ok(nickname) = db_request::nickname::get_nickname(account_key).await {
        let res = ResNickname { nickname };
        return send_response(Status::Ok, Some(res));
    }

    send_response(Status::InternalServerError, NONE_BODY)
}

static mut TEST_ACCOUNT_KEY: AccountKey = 123;

#[get("/test_account_new/<account_id>")]
pub async fn test_account_new(account_id: String) -> Custom<String> {
    // todo! account_id validate
    unsafe {
        let res = ResAccountKey {
            account_key: TEST_ACCOUNT_KEY,
        };
        if db_request::account_key::add_account_key(account_id, res.account_key)
            .await
            .is_ok()
        {
            TEST_ACCOUNT_KEY += 1;
            return send_response(Status::Ok, Some(res));
        }
    }

    send_response(Status::InternalServerError, NONE_BODY)
}
