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
mod video;
mod location;
mod unknown;

pub use self::text::TextMessage;
pub use self::image::ImageMessage;
pub use self::voice::VoiceMessage;
pub use self::shortvideo::ShortVideoMessage;
pub use self::video::VideoMessage;
pub use self::location::LocationMessage;
pub use self::unknown::UnknownMessage;

pub enum Message {
    TextMessage(TextMessage),
    ImageMessage(ImageMessage),
    VoiceMessage(VoiceMessage),
    ShortVideoMessage(ShortVideoMessage),
    VideoMessage(VideoMessage),
    LocationMessage(LocationMessage),
    UnknownMessage(UnknownMessage),
}
