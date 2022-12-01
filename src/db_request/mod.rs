use anyhow::bail;
use sqlx::{pool::PoolConnection, MySql};

use crate::{app, third_party::EMySQLType};

pub mod account_key;
pub mod model;
pub mod nickname;

pub(crate) async fn _get_account_db_pool() -> anyhow::Result<PoolConnection<MySql>> {
    if let Some(pool) = app::get_instance().get_mysql_pool(EMySQLType::ACCOUNT) {
        let a = pool.acquire().await?;
        return Ok(a);
    }
    bail!("failed get account db pool")
}
