use teloxide::{prelude::*, sugar::request::RequestReplyExt};

pub async fn handle_text(
    bot: Bot,
    message: Message
) -> ResponseResult<()> {
    bot.copy_message(message.chat.id, message.chat.id, message.id).reply_to(message.id).await?;
    Ok(())
}