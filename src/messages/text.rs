use super::super::xmlutil;
use super::MessageParser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TextMessage {
    source: String,
    target: String,
    time: usize,
    id: usize,
    content: String,
}

impl MessageParser for TextMessage {
    type WeChatMessage = TextMessage;

    fn source(&self) -> &str {
        &self.source
    }

    fn target(&self) -> &str {
        &self.target
    }

    fn time(&self) -> usize {
        self.time
    }

    fn id(&self) -> usize {
        self.id
    }

    ///
    /// ```
    /// extern crate wechat;
    ///
    /// use wechat::messages::MessageParser;
    /// use wechat::messages::TextMessage;
    ///
    /// fn main() {
    ///     let xml = "<xml>\
    ///     <ToUserName><![CDATA[toUser]]></ToUserName>\
    ///     <FromUserName><![CDATA[fromUser]]></FromUserName>\
    ///     <CreateTime>1348831860</CreateTime>\
    ///     <MsgType><![CDATA[text]]></MsgType>\
    ///     <Content><![CDATA[this is a test]]></Content>\
    ///     <MsgId>1234567890123456</MsgId>\
    ///     </xml>";
    ///     let msg = TextMessage::from_xml(xml);
    ///
    ///     assert_eq!("fromUser", msg.source());
    ///     assert_eq!("toUser", msg.target());
    ///     assert_eq!(1234567890123456, msg.id());
    ///     assert_eq!(1348831860, msg.time());
    ///     assert_eq!("this is a test", msg.content());
    /// }
    /// ```
    fn from_xml(xml: &str) -> TextMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()");
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()");
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()");
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()");
        let content = xmlutil::evaluate(&doc, "//xml/Content/text()");
        TextMessage {
            source: source.string(),
            target: target.string(),
            id: id.number() as usize,
            time: time.number() as usize,
            content: content.string(),
        }
    }
}

impl TextMessage {
    pub fn content(&self) -> &str {
        &self.content
    }
}