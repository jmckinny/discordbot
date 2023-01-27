use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    const LINK: &str = "https://github.com/jmckinny/frothybot";
    let mssg = format!("I am being rewritten in rust using the serenity framework!\n You can monitor my progress here <{LINK}>",);
    msg.author
        .direct_message(&ctx.http, |m| m.content(&mssg))
        .await?;
    msg.react(&ctx, '👌').await?;
    Ok(())
}
