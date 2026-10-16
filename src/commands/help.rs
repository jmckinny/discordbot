use crate::commands::types::{Context, Error};
use std::fmt::Write;

/// Show help message
#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"] command: Option<String>,
) -> Result<(), Error> {
    let commands = &ctx.framework().options().commands;

    let text = match command {
        Some(name) => match commands.iter().find(|c| c.name == name) {
            Some(cmd) => {
                let mut out = format!("**/{}**\n", cmd.name);
                out.push_str(cmd.description.as_deref().unwrap_or("No description."));
                if !cmd.subcommands.is_empty() {
                    out.push_str("\n\n**Subcommands:**\n");
                    for sub in &cmd.subcommands {
                        let _ = writeln!(
                            out,
                            "`{}`: {}",
                            sub.name,
                            sub.description.as_deref().unwrap_or("No description.")
                        );
                    }
                }
                out
            }
            None => format!("No command called `{name}`."),
        },
        None => {
            let mut out = String::from("**Commands**\n");
            for cmd in commands.iter().filter(|c| !c.hide_in_help) {
                let _ = writeln!(
                    out,
                    "`/{}`: {}",
                    cmd.name,
                    cmd.description.as_deref().unwrap_or("No description.")
                );
            }
            out.push_str("\nUse `/help <command>` for more info.");
            out
        }
    };

    ctx.send(poise::CreateReply::default().content(text).ephemeral(true))
        .await?;
    Ok(())
}
