pub mod chat_message;

use std::sync::mpsc::{Receiver, Sender};

use chat_message::ChatMessage;

pub fn run(channel_to_chat: Receiver<String>, send_to_app: Sender<ChatMessage>) {}
