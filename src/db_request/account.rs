use anyhow::bail;
use futures::TryStreamExt;
use sqlx::{pool::PoolConnection, MySql, Row};

use crate::{
    api::account_new::{AccountId, AccountKey, INVALID_ACCOUNT_KEY},
    app,
    third_party::EMySQLType,
};

pub async fn request_account_key(account_id: AccountId) -> anyhow::Result<AccountKey> {
    let mut conn = _get_account_db_pool().await?;
    let mut rows = sqlx::query("seLect account_key FROM web_account.account WHERE account_id = ?")
        .bind(account_id)
        .fetch(&mut conn);

    if let Some(row) = rows.try_next().await? {
        return Ok(row.try_get("account_key")?);
    }

    return Ok(INVALID_ACCOUNT_KEY);
}

pub async fn add_account_key(account_id: AccountId, account_key: AccountKey) -> anyhow::Result<()> {
    let mut conn = _get_account_db_pool().await?;
    sqlx::query("inSert INTO web_account.account(account_id, account_key) values(?, ?)")
        .bind(account_id)
        .bind(account_key)
        .execute(&mut conn)
        .await?;

    Ok(())
}

async fn _get_account_db_pool() -> anyhow::Result<PoolConnection<MySql>> {
    if let Some(pool) = app::get_instance().get_mysql_pool(EMySQLType::ACCOUNT) {
        let a = pool.acquire().await?;
        return Ok(a);
    }
    bail!("failed get account db pool")
}
