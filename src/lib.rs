mod bot;
pub mod chat_message;

use bot::Bot;
pub use chat_message::ChatMessage;
use eyre::Result;
use std::sync::mpsc::{Receiver, Sender};
pub use twitchchat;
use twitchchat::UserConfig;

pub fn run(
    name: String,
    token: String,
    channels: Vec<String>,
    receive_for_chat: Receiver<String>,
    send_incomming_chat_message: Sender<ChatMessage>,
) -> Result<()> {
    let user_config = get_user_config(name, token)?;
    let bot = Bot;

    // run the bot in the executor
    smol::run(async move {
        bot.run(
            &user_config,
            &channels,
            receive_for_chat,
            send_incomming_chat_message,
        )
        .await
    })
}

fn get_user_config(name: String, token: String) -> Result<twitchchat::UserConfig> {
    let config = UserConfig::builder()
        .name(name)
        .token(token)
        .enable_all_capabilities()
        .build()?;

    Ok(config)
}
