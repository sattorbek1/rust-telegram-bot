use teloxide::{prelude::*, sugar::request::RequestReplyExt};

pub async fn handle_start(
    bot: Bot,
    message: Message
) -> ResponseResult<()> {
    bot.send_message(message.chat.id, "Hell, world!").reply_to(message.id).await?;
    Ok(())
}