use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

mod start;
mod help;
pub mod text;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Start,
    #[command(description = "display help information.")]
    Help,
}

pub async fn handle_commands(bot: Bot, message: Message, command: Command) -> ResponseResult<()> {
    match command {
        Command::Start => start::handle_start(bot, message).await?,
        Command::Help => help::handle_help(bot, message).await?,
    }
    Ok(())
}