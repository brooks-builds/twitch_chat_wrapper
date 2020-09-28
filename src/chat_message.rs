use twitchchat::messages::Privmsg;

#[derive(Debug)]
pub struct ChatMessage {
    pub message: String,
    pub name: String,
    pub color_rgb: (u8, u8, u8),
    pub display_name: Option<String>,
    pub subscriber: bool,
}

impl ChatMessage {
    pub fn new(raw_message: Privmsg) -> ChatMessage {
        let name = raw_message.name().to_owned();
        let message = raw_message.data().to_owned();
        let color_rgb = raw_message
            .color()
            .map_or((0, 0, 0), |color| (color.rgb.0, color.rgb.1, color.rgb.2));
        let display_name = raw_message.display_name().map(|name| name.to_owned());
        let subscriber = raw_message.is_subscriber();

        ChatMessage {
            name,
            message,
            color_rgb,
            display_name,
            subscriber,
        }
    }

    /// Provided by twitch.tv/lordmzte
    /// uses bit shifting to convert a hex number to rgb
    fn _hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        //remove # at start
        let hex = hex.trim_start_matches('#');

        //Parse string as hex number (base 16)
        let hex_num = u32::from_str_radix(hex, 16).expect("invalid color string");

        //Integer overflow works in our favour here to essentially do the modulo for us.
        //Bit shifts to extract individual colors
        ((hex_num >> 16) as u8, (hex_num >> 8) as u8, hex_num as u8)
    }
}
