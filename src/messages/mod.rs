pub trait MessageParser {
    type WeChatMessage;

    fn source(&self) -> &str;
    fn target(&self) -> &str;
    fn time(&self) -> usize;
    fn id(&self) -> usize;
    fn from_xml(xml: &str) -> Self::WeChatMessage;
}


mod text;
pub use self::text::TextMessage;

pub enum Message {
    Text(TextMessage),
}
