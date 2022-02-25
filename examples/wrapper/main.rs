use std::{os::raw, sync::mpsc::channel, thread};

use eyre::Result;
use futures::StreamExt;
use irc::{
    client::{prelude::Config, Client},
    proto::{message, Command, Prefix},
};

#[tokio::main]
async fn main() -> Result<()> {
    let (send_to_chat, receive_from_app) = channel();
    let (send_to_app, receive_from_chat) = channel();

    tokio::spawn(async move {
        twitch_chat_wrapper::run(receive_from_app, send_to_app)
            .await
            .unwrap();
    });

    // tokio::spawn(async {
    //     let mut client = Client::new("config.toml").await.unwrap();
    //     client.identify().unwrap();
    //     let mut client_stream = client.stream().unwrap();
    //     while let Some(raw_message) = client_stream.next().await.transpose().unwrap() {
    //         if let Command::PRIVMSG(username, message) = raw_message.clone().command {
    //             if let Some(prefix) = raw_message.clone().prefix {
    //                 if let Prefix::Nickname(nickname, username, hostname) = prefix {
    //                     dbg!(nickname, username);
    //                 }
    //             }
    //             dbg!(username, message, raw_message);
    //         }
    //     }
    // });

    loop {
        if let Ok(chat_message) = receive_from_chat.try_recv() {
            dbg!(chat_message);
        }
    }
}
