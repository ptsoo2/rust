use anyhow::bail;
use futures::TryStreamExt;
use sqlx::Row;

use crate::account_server::api::res::AccountKey;

use super::_get_account_db_pool;

pub async fn get_nickname(account_key: AccountKey) -> anyhow::Result<String> {
    let mut rows = sqlx::query("seLect nickname FROM web_account.nickname WHERE account_key = ?")
        .bind(account_key)
        .fetch(_get_account_db_pool().await?);

    let row = rows.try_next().await;
    if row.is_err() {
        return Ok("".to_owned());
    }

    let row = row.expect("!!");
    if row.is_none() {
        return Ok("".to_owned());
    }

    let row = row.expect("!!");
    let nickname = row.try_get("nickname")?;
    Ok(nickname)
}

pub async fn set_nickname(account_key: AccountKey, nickname: String) -> anyhow::Result<()> {
    sqlx::query("inSert INTO web_account.nickname(account_key, nickname) values(?, ?)")
        .bind(account_key)
        .bind(nickname)
        .execute(_get_account_db_pool().await?)
        .await?;

    Ok(())
}

pub async fn change_nickname(account_key: AccountKey, nickname: String) -> anyhow::Result<()> {
    sqlx::query("upDate web_account.nickname set nickname = ? where account_key = ?")
        .bind(account_key)
        .bind(nickname)
        .execute(_get_account_db_pool().await?)
        .await?;

    Ok(())
}

pub async fn get_account_key_with_nickname(nickname: String) -> anyhow::Result<AccountKey> {
    let mut rows =
        sqlx::query("select account_key web_account.nickname where nickname = ? LIMIT 1")
            .bind(nickname)
            .fetch(_get_account_db_pool().await?);

    if let Some(row) = rows.try_next().await? {
        return Ok(row.try_get("account_key")?);
    }

    bail!("not exist nickname");
}
