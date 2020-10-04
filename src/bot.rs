use super::{ChatMessage, Result};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use twitchchat::{
    messages::Commands,
    runner::{AsyncRunner, Status},
    UserConfig,
};

pub struct Bot;

impl Bot {
    pub async fn run(
        &self,
        user_config: &UserConfig,
        channels: &Vec<String>,
        messages_for_chat: Receiver<String>,
        send_incomming_chat_message: Sender<ChatMessage>,
    ) -> Result<()> {
        let connector = twitchchat::connector::smol::Connector::twitch()?;

        let mut runner = AsyncRunner::connect(connector, user_config).await?;

        for channel in channels {
            runner.join(channel).await?;
        }

        self.main_loop(
            &mut runner,
            messages_for_chat,
            send_incomming_chat_message,
            channels.to_vec(),
        )
        .await
    }

    async fn main_loop<'a>(
        &self,
        runner: &mut AsyncRunner,
        messages_for_chat: Receiver<String>,
        send_incomming_chat_message: Sender<ChatMessage>,
        channels: Vec<String>,
    ) -> Result<()> {
        let mut writer = runner.writer();
        let _quit = runner.quit_handle();
        thread::spawn(move || -> Result<()> {
            loop {
                if let Ok(message) = messages_for_chat.recv() {
                    for channel in channels.iter() {
                        let message = twitchchat::commands::privmsg(channel, &message);
                        smol::block_on(writer.encode(message))?;
                    }
                }
            }
        });

        loop {
            match runner.next_message().await? {
                Status::Message(Commands::Privmsg(raw_message)) => {
                    send_incomming_chat_message.send(ChatMessage::new(raw_message))?;
                }
                Status::Quit | Status::Eof => break,
                Status::Message(_) => {
                    continue;
                }
            }
        }
        Ok(())
    }
}
