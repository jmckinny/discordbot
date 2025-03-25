use crate::commands::types::{Context, Error};
use crate::utils::database;
use poise::CreateReply;
use rand::seq::IndexedRandom;
use serenity::all::CreateEmbed;

const SLOTS_COST: u64 = 5;

/// Play the slot machine
#[poise::command(slash_command, prefix_command, category = "Games")]
pub async fn slots(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author();
    let tokens = database::remove_tokens(&ctx.data().db, user.id, SLOTS_COST).await;
    match tokens {
        Ok(_) => {
            let machine = SlotMachine::default();
            let embed = CreateEmbed::new()
                .title("Slot Machine")
                .description(machine.to_string());
            let reply = CreateReply::default().embed(embed);
            ctx.send(reply).await?;
            let winnings = machine.winnings();
            database::add_tokens(&ctx.data().db, user.id, winnings).await?;
            let msg = format!("You spent {SLOTS_COST} tokens and won {winnings} tokens!");
            ctx.reply(msg).await?;
            Ok(())
        }
        Err(e) => match e {
            database::DatabaseError::InsufficentTokens => {
                ctx.reply("Insufficent tokens").await?;
                Ok(())
            }
            _ => Err(e.into()),
        },
    }
}

struct SlotMachine {
    slots: Vec<String>,
}

impl SlotMachine {
    pub fn default() -> Self {
        let emoji_list: Vec<&str> = vec!["💀", "🤖", "💥", "🎱"];

        let mut rng = rand::rng();
        let mut slots = Vec::new();
        for _ in 0..4 {
            slots.push(emoji_list.choose(&mut rng).unwrap().to_string());
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
            20
        } else if matches >= 4 {
            60
        } else {
            0
        }
    }
}

impl std::fmt::Display for SlotMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slots = self.slots.join("-");
        writeln!(f, "{slots}")?;
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_slot_returns() {
        let mut winnings = 0;
        let iterations = 1000000;
        for _ in 0..iterations {
            let machine = SlotMachine::default();
            winnings += machine.winnings();
        }
        let per_play = winnings as f64 / iterations as f64;
        let expected_return_range = 4f64..4.3f64;
        assert!(expected_return_range.contains(&per_play));
    }
}
