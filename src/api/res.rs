use serde::Serialize;

pub type AccountKey = u64;
pub const INVALID_ACCOUNT_KEY: AccountKey = 0;
pub type AccountId = String;

#[derive(Debug, Serialize)]
pub struct ResExists {
    pub is_exist: bool,
}

#[derive(Debug, Serialize)]
pub struct ResAccountKey {
    pub account_key: AccountKey,
}

#[derive(Debug, Serialize)]
pub struct ResNickname {
    pub nickname: String,
}

pub const NONE_BODY: std::option::Option<()> = None::<()>;
