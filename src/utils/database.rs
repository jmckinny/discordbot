use anyhow::{Context, Result};
use serenity::model::prelude::UserId;
use sqlx::sqlite::SqlitePool;
use std::env;
use tracing::info;

use crate::Token;

pub async fn connect_to_db() -> Result<SqlitePool> {
    SqlitePool::connect(&env::var("DATABASE_URL")?)
        .await
        .context("Failed to open db connection")
}

pub async fn add_tokens(pool: &SqlitePool, user: UserId, tokens: Token) -> Result<()> {
    let user_id_num = i64::try_from(user.get())?;
    let token_amount = i64::try_from(tokens)?;
    sqlx::query!(
        r#"INSERT OR REPLACE INTO users (id, tokens) VALUES (?1, COALESCE((SELECT tokens FROM users WHERE id = ?1), 0) + ?2)"#,
        user_id_num,
        token_amount
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_tokens(pool: &SqlitePool, user: UserId) -> Result<Token> {
    let user_id_num = i64::try_from(user.get())?;
    let results = sqlx::query!(r#"SELECT tokens FROM users WHERE id = ?"#, user_id_num)
        .fetch_optional(pool)
        .await?;
    match results {
        Some(row) => Ok(row.tokens.unwrap_or_default() as u64),
        None => Ok(0),
    }
}
