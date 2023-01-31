use crate::utils::tokens::{add_tokens, remove_tokens};
use rand::seq::SliceRandom;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

const SLOTS_COST: u64 = 5;

#[command]
pub async fn slots(ctx: &Context, msg: &Message) -> CommandResult {
    if remove_tokens(ctx, msg.author.id, SLOTS_COST)
        .await?
        .is_none()
    {
        msg.reply(
            &ctx,
            format!("You do not have the required {SLOTS_COST} tokens!"),
        )
        .await?;
        return Ok(());
    }

    let slot_machine = SlotMachine::default();

    //Render slot machine
    msg.channel_id
        .send_message(&ctx, |m| {
            m.content("🎰Slot Machine🎰");
            m.reactions(slot_machine.slots.clone())
        })
        .await?;

    let winnings = slot_machine.winnings();
    add_tokens(ctx, msg.author.id, winnings).await?;
    msg.reply(
        &ctx,
        format!("You spent {SLOTS_COST} tokens and won {winnings} tokens!"),
    )
    .await?;

    Ok(())
}

struct SlotMachine {
    slots: Vec<char>,
}

impl SlotMachine {
    pub fn default() -> Self {
        let emoji_list: Vec<char> = vec!['💀', '🤖', '💥', '🎱'];

        let mut rng = rand::thread_rng();
        let mut slots = Vec::new();
        for _ in 0..4 {
            slots.push(*emoji_list.choose(&mut rng).unwrap());
        }
        SlotMachine { slots }
    }

    pub fn winnings(&self) -> u64 {
        let counts = self
            .slots
            .iter()
            .map(|item| self.slots.iter().filter(|f| f == &item).count())
            .collect::<Vec<usize>>();

        let matches = *counts.iter().max().unwrap();

        if matches >= 3 {
            10
        } else if matches >= 4 {
            25
        } else {
            0
        }
    }
}
