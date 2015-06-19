use time;

use xmlutil;
use messages::MessageParser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UnknownMessage {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: time::Tm,
    pub id: i64,
    pub raw: String,
}

impl MessageParser for UnknownMessage {
    type WeChatMessage = UnknownMessage;

    #[inline]
    fn from_xml(xml: &str) -> UnknownMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        UnknownMessage {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: time::at(time::Timespec::new(time, 0)),
            raw: xml.to_owned(),
        }
    }
}
