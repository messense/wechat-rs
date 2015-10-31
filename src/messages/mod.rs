pub trait MessageParser {
    type WeChatMessage;

    fn from_xml(xml: &str) -> Self::WeChatMessage;
}

mod text;
mod image;
mod voice;
mod shortvideo;
mod video;
mod location;
mod link;
mod unknown;

// export Message types
pub use self::text::TextMessage;
pub use self::image::ImageMessage;
pub use self::voice::VoiceMessage;
pub use self::shortvideo::ShortVideoMessage;
pub use self::video::VideoMessage;
pub use self::location::LocationMessage;
pub use self::link::LinkMessage;
pub use self::unknown::UnknownMessage;

// export Event types
pub use events::SubscribeEvent;
pub use events::UnsubscribeEvent;
pub use events::ScanEvent;
pub use events::SubscribeScanEvent;
pub use events::LocationEvent;
pub use events::ClickEvent;
pub use events::ViewEvent;
pub use events::QualificationVerifySuccessEvent;

// an enum or messages and events
#[derive(Debug, Clone)]
pub enum Message {
    TextMessage(TextMessage),
    ImageMessage(ImageMessage),
    VoiceMessage(VoiceMessage),
    ShortVideoMessage(ShortVideoMessage),
    VideoMessage(VideoMessage),
    LocationMessage(LocationMessage),
    LinkMessage(LinkMessage),
    UnknownMessage(UnknownMessage),
    SubscribeEvent(SubscribeEvent),
    UnsubscribeEvent(UnsubscribeEvent),
    ScanEvent(ScanEvent),
    SubscribeScanEvent(SubscribeScanEvent),
    LocationEvent(LocationEvent),
    ClickEvent(ClickEvent),
    ViewEvent(ViewEvent),
    QualificationVerifySuccessEvent(QualificationVerifySuccessEvent),
}

impl Message {
    pub fn parse<S: AsRef<str>>(xml: S) -> Message {
        use parser::parse_message;

        parse_message(xml.as_ref())
    }

    pub fn get_source(&self) -> String {
        match *self {
            Message::TextMessage(ref msg) => msg.source.to_owned(),
            Message::ImageMessage(ref msg) => msg.source.to_owned(),
            Message::VoiceMessage(ref msg) => msg.source.to_owned(),
            Message::ShortVideoMessage(ref msg) => msg.source.to_owned(),
            Message::VideoMessage(ref msg) => msg.source.to_owned(),
            Message::LocationMessage(ref msg) => msg.source.to_owned(),
            Message::LinkMessage(ref msg) => msg.source.to_owned(),
            Message::UnknownMessage(ref msg) => msg.source.to_owned(),
            Message::SubscribeEvent(ref msg) => msg.source.to_owned(),
            Message::UnsubscribeEvent(ref msg) => msg.source.to_owned(),
            Message::SubscribeScanEvent(ref msg) => msg.source.to_owned(),
            Message::ScanEvent(ref msg) => msg.source.to_owned(),
            Message::LocationEvent(ref msg) => msg.source.to_owned(),
            Message::ClickEvent(ref msg) => msg.source.to_owned(),
            Message::ViewEvent(ref msg) => msg.source.to_owned(),
            Message::QualificationVerifySuccessEvent(ref msg) => msg.source.to_owned(),
        }
    }

    pub fn get_target(&self) -> String {
        match *self {
            Message::TextMessage(ref msg) => msg.target.to_owned(),
            Message::ImageMessage(ref msg) => msg.target.to_owned(),
            Message::VoiceMessage(ref msg) => msg.target.to_owned(),
            Message::ShortVideoMessage(ref msg) => msg.target.to_owned(),
            Message::VideoMessage(ref msg) => msg.target.to_owned(),
            Message::LocationMessage(ref msg) => msg.target.to_owned(),
            Message::LinkMessage(ref msg) => msg.target.to_owned(),
            Message::UnknownMessage(ref msg) => msg.target.to_owned(),
            Message::SubscribeEvent(ref msg) => msg.target.to_owned(),
            Message::UnsubscribeEvent(ref msg) => msg.target.to_owned(),
            Message::SubscribeScanEvent(ref msg) => msg.target.to_owned(),
            Message::ScanEvent(ref msg) => msg.target.to_owned(),
            Message::LocationEvent(ref msg) => msg.target.to_owned(),
            Message::ClickEvent(ref msg) => msg.target.to_owned(),
            Message::ViewEvent(ref msg) => msg.target.to_owned(),
            Message::QualificationVerifySuccessEvent(ref msg) => msg.target.to_owned(),
        }
    }
}
