use crate::commands::types::{Context, Error};

const LINK: &str = "https://github.com/jmckinny/frothybot";

#[poise::command(slash_command, prefix_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let mssg = format!(
        "I am being rewritten in rust using the serenity framework!\n You can monitor my progress here <{LINK}>",
    );
    ctx.reply(mssg).await?;
    Ok(())
}
