use super::super::xmlutil;
use super::{MessageParser, MessageData};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VoiceMessage {
    source: String,
    target: String,
    time: i64,
    id: i64,
    media_id: String,
    format: String,
    recognition: String,
}

impl MessageData for VoiceMessage {
    fn source(&self) -> &str {
        &self.source
    }

    fn target(&self) -> &str {
        &self.target
    }

    fn time(&self) -> i64 {
        self.time
    }

    fn id(&self) -> i64 {
        self.id
    }
}

impl MessageParser for VoiceMessage {
    type WeChatMessage = VoiceMessage;

    fn from_xml(xml: &str) -> VoiceMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()");
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()");
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()");
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()");
        let media_id = xmlutil::evaluate(&doc, "//xml/MediaId/text()");
        let format = xmlutil::evaluate(&doc, "//xml/Format/text()");
        let recognition = xmlutil::evaluate(&doc, "//xml/Recognition/text()");
        VoiceMessage {
            source: source.string(),
            target: target.string(),
            id: id.number() as i64,
            time: time.number() as i64,
            media_id: media_id.string(),
            format: format.string(),
            recognition: recognition.string(),
        }
    }
}

impl VoiceMessage {
    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn media_id(&self) -> &str {
        &self.media_id
    }

    pub fn recognition(&self) -> &str {
        &self.recognition
    }
}

#[cfg(test)]
mod tests {
    use messages::{MessageParser, MessageData};
    use super::VoiceMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[text]]></MsgType>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <Format><![CDATA[Format]]></Format>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = VoiceMessage::from_xml(xml);

        assert_eq!("fromUser", msg.source());
        assert_eq!("toUser", msg.target());
        assert_eq!(1234567890123456, msg.id());
        assert_eq!(1348831860, msg.time());
        assert_eq!("media_id", msg.media_id());
        assert_eq!("Format", msg.format());
        assert_eq!("", msg.recognition());
    }
}
