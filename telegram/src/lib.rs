use teloxide::prelude::*;

const MIKE_CHAT_ID: i64 = 248923795;

pub async fn run_telegram_bot(){
    let bot = Bot::new("5796751667:AAGPS3QNCcqIYJJuB7C91Wzkc2MAR2w7UCQ");

    let _ = bot.send_message(ChatId(MIKE_CHAT_ID), "wazzap").await;

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        println!("{:?}", msg);
        // bot.send_dice(msg.chat.id).await?;
        if let Some(txt) = msg.text() {
            let _ = bot.send_message(msg.chat.id, txt).await;
        }
        Ok(())
    }).await;
}