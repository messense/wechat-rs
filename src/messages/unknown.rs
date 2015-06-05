use super::super::xmlutil;
use super::{MessageParser, MessageData};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UnknownMessage {
    source: String,
    target: String,
    time: i64,
    id: i64,
}

impl MessageData for UnknownMessage {
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

impl MessageParser for UnknownMessage {
    type WeChatMessage = UnknownMessage;

    fn from_xml(xml: &str) -> UnknownMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()");
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()");
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()");
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()");
        UnknownMessage {
            source: source.string(),
            target: target.string(),
            id: id.number() as i64,
            time: time.number() as i64,
        }
    }
}
