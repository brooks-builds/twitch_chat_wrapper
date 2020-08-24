mod bot;
pub mod chat_message;

use bot::Bot;
pub use chat_message::ChatMessage;
use eyre::Result;
use std::sync::mpsc::{Receiver, Sender};
use twitchchat::UserConfig;

pub fn run(
    receive_for_chat: Receiver<String>,
    send_incomming_chat_message: Sender<ChatMessage>,
) -> Result<()> {
    dotenv::dotenv().ok();
    let user_config = get_user_config()?;
    let channels = channels_to_join()?;
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

fn get_env_var(key: &str) -> Result<String> {
    let my_var = std::env::var(key)?;
    Ok(my_var)
}

fn channels_to_join() -> Result<Vec<String>> {
    let channels = get_env_var("TWITCH_CHANNEL")?
        .split(',')
        .map(ToString::to_string)
        .collect();
    Ok(channels)
}

fn get_user_config() -> Result<twitchchat::UserConfig> {
    let name = get_env_var("TWITCH_NAME")?;
    let token = get_env_var("TWITCH_TOKEN")?;

    let config = UserConfig::builder()
        .name(name)
        .token(token)
        .enable_all_capabilities()
        .build()?;

    Ok(config)
}
