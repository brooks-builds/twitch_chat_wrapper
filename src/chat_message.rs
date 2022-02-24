#[derive(Debug)]
pub struct ChatMessage {
    pub message: String,
    pub name: String,
    pub color_rgb: (u8, u8, u8),
    pub display_name: Option<String>,
    pub subscriber: bool,
}

impl ChatMessage {
    pub fn new() -> ChatMessage {
        ChatMessage {
            name: "someone".into(),
            message: "I am a message".into(),
            color_rgb: (200, 200, 200),
            display_name: Some("Not a bot".into()),
            subscriber: false,
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
