use crate::db_request::{self};

pub type AccountKey = u64;
pub const INVALID_ACCOUNT_KEY: AccountKey = 0;
pub type AccountId = String;

#[get("/account_exists/<account_id>")]
pub async fn account_exists(account_id: String) -> String {
    // todo! account_id validate
    if let Ok(account_key) = db_request::account::request_account_key(account_id).await {
        return if account_key == INVALID_ACCOUNT_KEY {
            "not exist".to_string()
        } else {
            "exist".to_string()
        };
    }

    "failed execute".to_string()
}

#[get("/account_new/<account_id>")]
pub async fn account_new(account_id: String) -> String {
    // todo! account_id validate
    let account_key = 0;
    if db_request::account::add_account_key(account_id, account_key)
        .await
        .is_ok()
    {
        return account_key.to_string();
    }

    "failed execute".to_string()
}

static mut TEST_ACCOUNT_KEY: AccountKey = 123;

#[get("/test_account_new/<account_id>")]
pub async fn test_account_new(account_id: String) -> String {
    // todo! account_id validate
    unsafe {
        if db_request::account::add_account_key(account_id, TEST_ACCOUNT_KEY)
            .await
            .is_ok()
        {
            let prev_account_key = TEST_ACCOUNT_KEY;
            TEST_ACCOUNT_KEY += 1;
            return prev_account_key.to_string();
        }
    }

    "failed execute".to_string()
}
