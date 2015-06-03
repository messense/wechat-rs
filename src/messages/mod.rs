pub trait MessageParser {
    type WeChatMessage;

    fn from_xml(xml: &str) -> Self::WeChatMessage;
}

pub trait MessageData {
    fn source(&self) -> &str;
    fn target(&self) -> &str;
    fn time(&self) -> usize;
    fn id(&self) -> usize;
}


mod text;
mod image;
mod voice;
mod shortvideo;
mod unknown;

pub use self::text::TextMessage;
pub use self::image::ImageMessage;
pub use self::voice::VoiceMessage;
pub use self::shortvideo::ShortVideoMessage;
pub use self::unknown::UnknownMessage;

pub enum Message {
    Text(TextMessage),
    Image(ImageMessage),
    Voice(VoiceMessage),
    ShortVideo(ShortVideoMessage),
    Unknown(UnknownMessage),
}
