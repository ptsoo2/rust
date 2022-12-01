use anyhow::bail;
use futures::TryStreamExt;
use sqlx::Row;

use crate::{api::res::AccountKey, db_request::_get_account_db_pool};

pub async fn get_nickname(account_key: AccountKey) -> anyhow::Result<String> {
    let mut conn = _get_account_db_pool().await?;
    let mut rows = sqlx::query("seLect nickname FROM web_account.nickname WHERE account_key = ?")
        .bind(account_key)
        .fetch(&mut conn);

    if let Some(row) = rows.try_next().await? {
        match row.try_get("nickname") {
            Ok(nickname) => return Ok(nickname),
            Err(_) => return Ok("".to_owned()),
        }
    }

    bail!("not exist account")
}

pub async fn set_nickname(account_key: AccountKey, nickname: String) -> anyhow::Result<()> {
    let mut conn = _get_account_db_pool().await?;
    sqlx::query("inSert INTO web_account.nickname(account_key, nickname) values(?, ?)")
        .bind(account_key)
        .bind(nickname)
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn change_nickname(account_key: AccountKey, nickname: String) -> anyhow::Result<()> {
    let mut conn = _get_account_db_pool().await?;
    sqlx::query("upDate web_account.nickname set nickname = ? where account_key = ?")
        .bind(account_key)
        .bind(nickname)
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn get_account_key_with_nickname(nickname: String) -> anyhow::Result<AccountKey> {
    let mut conn = _get_account_db_pool().await?;
    let mut rows =
        sqlx::query("select account_key web_account.nickname where nickname = ? LIMIT 1")
            .bind(nickname)
            .fetch(&mut conn);

    if let Some(row) = rows.try_next().await? {
        return Ok(row.try_get("account_key")?);
    }

    bail!("not exist nickname");
}
