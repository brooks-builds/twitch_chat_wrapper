use twitchchat::messages::Privmsg;

#[derive(Debug)]
pub struct ChatMessage {
    message: String,
    name: String
}

impl ChatMessage {
    pub fn new(raw_message: Privmsg) -> ChatMessage {
        let name = raw_message.name().to_owned();
        let message = raw_message.data().to_owned();

        ChatMessage {
            name,
            message
        }
    }
}