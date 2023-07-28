use anyhow::bail;
use sqlx::MySqlPool;

use crate::{app, third_party::EMySQLType};

pub mod account_key;
pub mod nickname;

pub(crate) async fn _get_account_db_pool() -> anyhow::Result<&'static MySqlPool> {
    if let Some(pool) = app::get_instance().get_mysql_pool(EMySQLType::ACCOUNT) {
        return Ok(pool);
    }
    bail!("failed get account db pool")
}
