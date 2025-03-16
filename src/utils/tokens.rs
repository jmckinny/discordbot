use crate::{
    Token,
    commands::types::{Context, Error},
};
use poise::serenity_prelude as serenity;
use std::fmt;

#[derive(Debug)]
pub struct InsufficentTokensError;

impl std::error::Error for InsufficentTokensError {}

impl fmt::Display for InsufficentTokensError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not enough tokens!")
    }
}

pub fn get_tokens(ctx: Context<'_>, user: &serenity::User) -> Token {
    let data = ctx.data();
    let token_count = data
        .tokens
        .read()
        .expect("Failed to aquire token read lock");
    let token_count = token_count.get(&user.id);
    *token_count.unwrap_or(&0u64)
}
pub async fn add_tokens(
    ctx: Context<'_>,
    user: &serenity::User,
    amount: Token,
) -> Result<Token, Error> {
    let data = ctx.data();
    let mut token_counter = data
        .tokens
        .write()
        .expect("Failed to aquire token write lock");
    if let Some(count) = token_counter.get_mut(&user.id) {
        *count += amount;
        Ok(*count)
    } else {
        token_counter.insert(user.id, amount);
        Ok(amount)
    }
}
pub async fn remove_tokens(
    ctx: Context<'_>,
    user: &serenity::User,
    amount: Token,
) -> Result<Token, Error> {
    let data = ctx.data();
    let mut token_counter = data
        .tokens
        .write()
        .expect("Failed to aquire token write lock");
    if let Some(count) = token_counter.get_mut(&user.id) {
        if amount > *count {
            // User is too broke
            return Err(Box::new(InsufficentTokensError));
        }
        *count -= amount;
        Ok(*count)
    } else {
        // User DNE so they have no tokens
        Err(Box::new(InsufficentTokensError))
    }
}
