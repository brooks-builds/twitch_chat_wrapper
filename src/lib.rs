pub mod chat_message;

use std::sync::mpsc::{Receiver, Sender};

use chat_message::ChatMessage;
use eyre::{bail, Result};
use futures::StreamExt;
use irc::{
    client::Client,
    proto::{Command, Prefix},
};

pub async fn run(
    channel_to_chat: Receiver<String>,
    send_to_app: Sender<ChatMessage>,
) -> Result<()> {
    let mut client = match Client::new("config.toml").await {
        Ok(client) => client,
        Err(_error) => bail!("Error loading the config.toml for twitch chat"),
    };

    client.identify()?;
    let mut client_stream = client.stream()?;
    while let Some(raw_message) = client_stream.next().await.transpose()? {
        if let Command::PRIVMSG(_logged_in_as, message) = raw_message.clone().command {
            let (display_name, username) = if let Some(prefix) = raw_message.clone().prefix {
                if let Prefix::Nickname(nickname, username, _hostname) = prefix {
                    if !nickname.is_empty() {
                        (Some(nickname), username)
                    } else {
                        (None, username)
                    }
                } else {
                    (None, "anonymous".to_owned())
                }
            } else {
                (None, "anonymous".to_owned())
            };
            let chat_message = ChatMessage::new(username, display_name, message);
            dbg!("sending chat message to app");
            send_to_app.send(chat_message)?;
        }
    }

    Ok(())
}
