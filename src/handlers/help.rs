use teloxide::{prelude::*, sugar::request::RequestReplyExt, utils::command::BotCommands};

pub async fn handle_help(
    bot: Bot,
    message: Message
) -> ResponseResult<()> {
    bot.send_message(message.chat.id, super::Command::descriptions().to_string()).reply_to(message.id).await?;
    Ok(())
}