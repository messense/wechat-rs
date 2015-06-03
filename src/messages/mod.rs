pub trait MessageParser {
    type WeChatMessage;

    fn source(&self) -> &str;
    fn target(&self) -> &str;
    fn time(&self) -> usize;
    fn id(&self) -> usize;
    fn from_xml(xml: &str) -> Self::WeChatMessage;
}


mod text;
mod image;

pub use self::text::TextMessage;
pub use self::image::ImageMessage;

pub enum Message {
    Text(TextMessage),
    Image(ImageMessage),
}
