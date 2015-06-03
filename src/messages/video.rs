use super::super::xmlutil;
use super::{MessageParser, MessageData};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VideoMessage {
    source: String,
    target: String,
    time: usize,
    id: usize,
    media_id: String,
    thumb_media_id: String,
}

impl MessageData for VideoMessage {
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
}

impl MessageParser for VideoMessage {
    type WeChatMessage = VideoMessage;

    fn from_xml(xml: &str) -> VideoMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()");
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()");
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()");
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()");
        let media_id = xmlutil::evaluate(&doc, "//xml/MediaId/text()");
        let thumb_media_id = xmlutil::evaluate(&doc, "//xml/ThumbMediaId/text()");
        VideoMessage {
            source: source.string(),
            target: target.string(),
            id: id.number() as usize,
            time: time.number() as usize,
            media_id: media_id.string(),
            thumb_media_id: thumb_media_id.string(),
        }
    }
}

impl VideoMessage {
    pub fn media_id(&self) -> &str {
        &self.media_id
    }

    pub fn thumb_media_id(&self) -> &str {
        &self.thumb_media_id
    }
}

#[cfg(test)]
mod tests {
    use messages::{MessageParser, MessageData};
    use super::VideoMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[text]]></MsgType>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = VideoMessage::from_xml(xml);

        assert_eq!("fromUser", msg.source());
        assert_eq!("toUser", msg.target());
        assert_eq!(1234567890123456, msg.id());
        assert_eq!(1348831860, msg.time());
        assert_eq!("media_id", msg.media_id());
        assert_eq!("thumb_media_id", msg.thumb_media_id());
    }
}
