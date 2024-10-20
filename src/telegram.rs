use std::error::Error;
use teloxide::prelude::*;

pub async fn create_telegram_bot(bot_token: &str) -> Result<Bot, Box<dyn Error>> {
    let bot = Bot::new(bot_token);

    match bot.get_me().await {
        Ok(me) => {
            println!(
                "Telegram bot connected successfully. Bot username: @{}",
                me.username()
            );
            Ok(bot)
        }
        Err(e) => Err(format!("Failed to connect to Telegram bot: {}", e).into()),
    }
}

pub async fn send_telegram_message(
    bot: &Bot,
    chat_id: i64,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    bot.send_message(ChatId(chat_id), message).await?;
    Ok(())
}
