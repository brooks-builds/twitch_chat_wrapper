use std::env;
use tokio::stream::StreamExt as _;
use twitchchat::{events, messages, Control, Dispatcher, IntoChannel, Runner, Status, Writer};

pub struct TwitchChatWrapper {}

impl TwitchChatWrapper {
    pub fn new() -> TwitchChatWrapper {
        dotenv::dotenv().ok();
        TwitchChatWrapper {}
    }

    pub async fn run(&self) {
        let dispatcher = Dispatcher::new();
        let (mut runner, mut control) = Runner::new(dispatcher.clone());

        let connector = twitchchat::Connector::new(|| async move {
            let twitch_nickname = env::var("TWITCH_NICKNAME").unwrap();
            let twitch_password = env::var("TWITCH_PASSWORD").unwrap();
            twitchchat::native_tls::connect_easy(&twitch_nickname, &twitch_password).await
        });
        let done = runner.run_to_completion(connector).await;

        self.start(dispatcher, "brookzerker", control.writer());
    }

    async fn start(&self, dispatcher: Dispatcher, channel: impl IntoChannel, writer: &mut Writer) {
        // subscribe to the events we're interested in
        let mut events = dispatcher.subscribe::<events::Privmsg>();

        // and wait for a specific event (blocks the current task)
        let ready = dispatcher.wait_for::<events::IrcReady>().await.unwrap();
        eprintln!("connected! our name is: {}", ready.nickname);

        // and then join a channel
        eprintln!("joining our channel");
        writer.join(channel).await.unwrap();

        // and then our 'main loop'
        while let Some(msg) = events.next().await {
            dbg!(msg);
        }
    }
}
