use time;

use xmlutil;
use messages::MessageParser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ScanEvent {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub create_time: time::Tm,
    pub id: i64,
    pub scene_id: String,
    pub ticket: String,
    pub event: String,
    pub raw: String,
}

impl MessageParser for ScanEvent {
    type WeChatMessage = ScanEvent;

    #[inline]
    fn from_xml(xml: &str) -> ScanEvent {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()").string();
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()").string();
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()").number() as i64;
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()").number() as i64;
        let scene_id = xmlutil::evaluate(&doc, "//xml/EventKey/text()").string();
        let ticket = xmlutil::evaluate(&doc, "//xml/Ticket/text()").string();
        ScanEvent {
            source: source,
            target: target,
            id: id,
            time: time,
            create_time: time::at(time::Timespec::new(time, 0)),
            scene_id: scene_id,
            ticket: ticket,
            event: "scan".to_owned(),
            raw: xml.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use messages::MessageParser;
    use super::ScanEvent;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>123456789</CreateTime>\
        <MsgType><![CDATA[event]]></MsgType>\
        <Event><![CDATA[SCAN]]></Event>\
        <EventKey><![CDATA[SCENE_VALUE]]></EventKey>\
        <Ticket><![CDATA[TICKET]]></Ticket>\
        </xml>";
        let msg = ScanEvent::from_xml(xml);

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!("scan", &msg.event);
        assert_eq!(123456789, msg.time);
        assert_eq!("SCENE_VALUE", &msg.scene_id);
        assert_eq!("TICKET", &msg.ticket);
    }
}