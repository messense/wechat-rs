use super::super::xmlutil;
use super::MessageParser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ShortVideoMessage {
    source: String,
    target: String,
    time: usize,
    id: usize,
    media_id: String,
    thumb_media_id: String,
}

impl MessageParser for ShortVideoMessage {
    type WeChatMessage = ShortVideoMessage;

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
    /// use wechat::messages::ShortVideoMessage;
    ///
    /// fn main() {
    ///     let xml = "<xml>\
    ///     <ToUserName><![CDATA[toUser]]></ToUserName>\
    ///     <FromUserName><![CDATA[fromUser]]></FromUserName>\
    ///     <CreateTime>1348831860</CreateTime>\
    ///     <MsgType><![CDATA[text]]></MsgType>\
    ///     <MediaId><![CDATA[media_id]]></MediaId>\
    ///     <ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>\
    ///     <MsgId>1234567890123456</MsgId>\
    ///     </xml>";
    ///     let msg = ShortVideoMessage::from_xml(xml);
    ///
    ///     assert_eq!("fromUser", msg.source());
    ///     assert_eq!("toUser", msg.target());
    ///     assert_eq!(1234567890123456, msg.id());
    ///     assert_eq!(1348831860, msg.time());
    ///     assert_eq!("media_id", msg.media_id());
    ///     assert_eq!("thumb_media_id", msg.thumb_media_id());
    /// }
    /// ```
    fn from_xml(xml: &str) -> ShortVideoMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()");
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()");
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()");
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()");
        let media_id = xmlutil::evaluate(&doc, "//xml/MediaId/text()");
        let thumb_media_id = xmlutil::evaluate(&doc, "//xml/ThumbMediaId/text()");
        ShortVideoMessage {
            source: source.string(),
            target: target.string(),
            id: id.number() as usize,
            time: time.number() as usize,
            media_id: media_id.string(),
            thumb_media_id: thumb_media_id.string(),
        }
    }
}

impl ShortVideoMessage {
    pub fn media_id(&self) -> &str {
        &self.media_id
    }

    pub fn thumb_media_id(&self) -> &str {
        &self.thumb_media_id
    }
}