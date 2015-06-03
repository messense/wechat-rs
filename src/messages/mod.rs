pub trait WeChatMessage {
    fn source(&self) -> &str;
    fn target(&self) -> &str;
    fn time(&self) -> usize;
    fn id(&self) -> usize;
    fn from_xml<T>(xml: &str) -> T where T: WeChatMessage;
}


mod text;
pub use self::text::TextMessage;

pub enum Message {
    Text(TextMessage),
}
