use twitchchat::{runner::{AsyncRunner, Status}, UserConfig, messages::AllCommands};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use super::Result;

pub struct Bot;

impl Bot {
    // run the bot until its done
    pub async fn run(&self, user_config: &UserConfig, channels: &[String], messages_for_chat: Receiver<String>, send_incomming_chat_message: Sender<String>) -> Result<()> {
        let connector = twitchchat::connector::smol::Connector::twitch();

        let mut runner = AsyncRunner::connect(connector, user_config).await?;
        println!("connecting, we are: {}", runner.identity.username());

        for channel in channels {
            println!("joining: {}", channel);
            runner.join(channel).await?;
        }

        println!("starting main loop");
        self.main_loop(&mut runner, messages_for_chat, send_incomming_chat_message).await
    }

    // the main loop of the bot
    async fn main_loop(&self, runner: &mut AsyncRunner, messages_for_chat: Receiver<String>, send_incomming_chat_message: Sender<String>) -> Result<()> {
        // this is clonable, but we can just share it via &mut
        // this is rate-limited writer
        let mut writer = runner.writer();
        // this is clonable, but using it consumes it.
        // this is used to 'quit' the main loop
        let _quit = runner.quit_handle();
        thread::spawn(move || -> Result<()> {
            loop {
                if let Ok(message) = messages_for_chat.recv() {
                    println!("received message: {}", &message);
                    let message = twitchchat::commands::privmsg("brookzerker", &message);
                    smol::block_on(writer.encode(message))?;
                }
            }
        });

        loop {
            // this drives the internal state of the crate
            match runner.next_message().await? {
                // if we get a Privmsg (you'll get an AllCommands enum for all messages received)
                Status::Message(AllCommands::Privmsg(pm)) => {
                    // see if its a command and do stuff with it
                    send_incomming_chat_message.send(pm.data().to_owned())?;
                }
                // stop if we're stopping
                Status::Quit | Status::Eof => break,
                // ignore the rest
                Status::Message(..) => continue,
            }
        }

        println!("end of main loop");
        Ok(())
    }
}