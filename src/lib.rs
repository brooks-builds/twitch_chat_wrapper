pub mod chat_message;
pub use chat_message::ChatMessage;

// note this uses `smol`. you can use `tokio` or `async_std` or `async_io` if you prefer.


use eyre::Result;

use std::sync::mpsc::{Receiver, Sender};

// extensions to the Privmsg type
use twitchchat::{
    UserConfig,
};

mod bot;
use bot::Bot;

pub fn run(receive_for_chat: Receiver<String>, send_incomming_chat_message: Sender<ChatMessage>) -> Result<()> {
    dotenv::dotenv().ok();
    // you'll need a user configuration
    let user_config = get_user_config()?;
    // and some channels to join
    let channels = channels_to_join()?;

    let bot = Bot;

    // run the bot in the executor
    smol::run(async move { bot.run(&user_config, &channels, receive_for_chat, send_incomming_chat_message).await })
}

// some helpers for the demo
fn get_env_var(key: &str) -> Result<String> {
    let my_var = std::env::var(key)?;
    Ok(my_var)
}

// channels can be either in the form of '#museun' or 'museun'. the crate will internally add the missing #
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

    // you need a `UserConfig` to connect to Twitch
    let config = UserConfig::builder()
        // the name of the associated twitch account
        .name(name)
        // and the provided OAuth token
        .token(token)
        // and enable all of the advanced message signaling from Twitch
        .enable_all_capabilities()
        .build()?;

    Ok(config)
}
