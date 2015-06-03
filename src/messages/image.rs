use super::super::xmlutil;
use super::MessageParser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ImageMessage {
    source: String,
    target: String,
    time: usize,
    id: usize,
    media_id: String,
    image: String,
}

impl MessageParser for ImageMessage {
    type WeChatMessage = ImageMessage;

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
    /// use wechat::messages::ImageMessage;
    ///
    /// fn main() {
    ///     let xml = "<xml>\
    ///     <ToUserName><![CDATA[toUser]]></ToUserName>\
    ///     <FromUserName><![CDATA[fromUser]]></FromUserName>\
    ///     <CreateTime>1348831860</CreateTime>\
    ///     <MsgType><![CDATA[image]]></MsgType>\
    ///     <PicUrl><![CDATA[this is a url]]></PicUrl>\
    ///     <MediaId><![CDATA[media_id]]></MediaId>\
    ///     <MsgId>1234567890123456</MsgId>\
    ///     </xml>";
    ///     let msg = ImageMessage::from_xml(xml);
    ///
    ///     assert_eq!("fromUser", msg.source());
    ///     assert_eq!("toUser", msg.target());
    ///     assert_eq!(1234567890123456, msg.id());
    ///     assert_eq!(1348831860, msg.time());
    ///     assert_eq!("media_id", msg.media_id());
    ///     assert_eq!("this is a url", msg.image());
    /// }
    /// ```
    fn from_xml(xml: &str) -> ImageMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()");
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()");
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()");
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()");
        let media_id = xmlutil::evaluate(&doc, "//xml/MediaId/text()");
        let image = xmlutil::evaluate(&doc, "//xml/PicUrl/text()");
        ImageMessage {
            source: source.string(),
            target: target.string(),
            id: id.number() as usize,
            time: time.number() as usize,
            media_id: media_id.string(),
            image: image.string(),
        }
    }
}

impl ImageMessage {
    pub fn media_id(&self) -> &str {
        &self.media_id
    }

    pub fn image(&self) -> &str {
        &self.image
    }
}
