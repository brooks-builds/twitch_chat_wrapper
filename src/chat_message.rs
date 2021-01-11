use twitchchat::messages::Privmsg;

#[derive(Debug, Default)]
pub struct ColorRgb(u8, u8, u8);

pub struct ChatMessageBuilder {
    message: String,
    name: String,
    color: Option<ColorRgb>,
    display_name: Option<String>,
    subscriber: bool,
}

impl ChatMessageBuilder {
    pub fn new(name: String, message: String) -> Self {
        ChatMessageBuilder {
            message,
            name,
            color: None,
            display_name: None,
            subscriber: false,
        }
    }

    pub fn color(mut self, color: ColorRgb) -> Self {
        self.color = Some(color);
        self
    }

    pub fn display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }

    pub fn subscriber(mut self) -> Self {
        self.subscriber = true;
        self
    }

    pub fn build(self) -> ChatMessage {
        ChatMessage {
            message: self.message,
            name: self.name,
            color: self.color.unwrap_or_default(),
            display_name: self.display_name,
            subscriber: self.subscriber,
        }
    }
}

#[derive(Debug)]
pub struct ChatMessage {
    pub message: String,
    pub name: String,
    pub color: ColorRgb,
    pub display_name: Option<String>,
    pub subscriber: bool,
}

impl ChatMessage {
    pub fn builder(name: String, message: String) -> ChatMessageBuilder {
        ChatMessageBuilder::new(name, message)
    }

    pub fn new(name: String, message: String, color: ColorRgb, display_name: Option<String>, subscriber: bool) -> ChatMessage {
        ChatMessage {
            message,
            name,
            color,
            display_name,
            subscriber,
        }
    }

    /// Provided by twitch.tv/lordmzte
    /// uses bit shifting to convert a hex number to rgb
    fn _hex_to_rgb(hex: &str) -> ColorRgb {
        //remove # at start
        let hex = hex.trim_start_matches('#');

        //Parse string as hex number (base 16)
        let hex_num = u32::from_str_radix(hex, 16).expect("invalid color string");

        //Integer overflow works in our favour here to essentially do the modulo for us.
        //Bit shifts to extract individual colors
        ColorRgb((hex_num >> 16) as u8, (hex_num >> 8) as u8, hex_num as u8)
    }
}

impl<'a> From<Privmsg<'a>> for ChatMessage {
    fn from(raw_message: Privmsg) -> Self {
        let name = raw_message.name().to_owned();
        let message = raw_message.data().to_owned();
        let (r, g, b) = raw_message
            .color()
            .map_or((0, 0, 0), |color| (color.rgb.0, color.rgb.1, color.rgb.2));
        let display_name = raw_message.display_name().map(|name| name.to_owned());
        let subscriber = raw_message.is_subscriber();

        ChatMessage {
            name,
            message,
            color: ColorRgb(r, g, b),
            display_name,
            subscriber,
        }
    }
}
