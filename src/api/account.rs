use rocket::{http::Status, response::status::Custom};

use crate::db_request::{self};

use super::{
    common::send_response,
    res::{AccountKey, ResAccountExists, ResAccountKey, INVALID_ACCOUNT_KEY, NONE_BODY},
};

#[get("/exists/<account_id>")]
pub async fn account_exists(account_id: String) -> Custom<String> {
    // todo! account_id validate
    let mut res = ResAccountExists { is_exist: false };
    if let Ok(account_key) = db_request::account::get_account_key(account_id).await {
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
        account_key: 0123123123,
    };

    if db_request::account::add_account_key(account_id, res.account_key)
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
    if let Ok(account_key) = db_request::account::get_account_key(account_id).await {
        if account_key != INVALID_ACCOUNT_KEY {
            let res = ResAccountKey {
                account_key: account_key,
            };
            return send_response(Status::Ok, Some(res));
        }
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
        if db_request::account::add_account_key(account_id, res.account_key)
            .await
            .is_ok()
        {
            TEST_ACCOUNT_KEY += 1;
            return send_response(Status::Ok, Some(res));
        }
    }

    send_response(Status::InternalServerError, NONE_BODY)
}
