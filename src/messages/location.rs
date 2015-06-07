use super::super::xmlutil;
use super::{MessageParser, MessageData};

#[derive(Debug, PartialEq, Clone)]
pub struct LocationMessage {
    source: String,
    target: String,
    time: i64,
    id: i64,
    location_x: f64,
    location_y: f64,
    scale: usize,
    label: String,
}

impl MessageData for LocationMessage {
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

impl MessageParser for LocationMessage {
    type WeChatMessage = LocationMessage;

    fn from_xml(xml: &str) -> LocationMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()");
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()");
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()");
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()");
        let location_x = xmlutil::evaluate(&doc, "//xml/Location_X/text()");
        let location_y = xmlutil::evaluate(&doc, "//xml/Location_Y/text()");
        let scale = xmlutil::evaluate(&doc, "//xml/Scale/text()");
        let label = xmlutil::evaluate(&doc, "//xml/Label/text()");
        LocationMessage {
            source: source.string(),
            target: target.string(),
            id: id.number() as i64,
            time: time.number() as i64,
            location_x: location_x.number(),
            location_y: location_y.number(),
            scale: scale.number() as usize,
            label: label.string(),
        }
    }
}

impl LocationMessage {
    pub fn location_x(&self) -> f64 {
        self.location_x
    }

    pub fn location_y(&self) -> f64 {
        self.location_y
    }

    pub fn location(&self) -> (f64, f64) {
        (self.location_x, self.location_y)
    }

    pub fn scale(&self) -> usize {
        self.scale
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[cfg(test)]
mod tests {
    use messages::{MessageParser, MessageData};
    use super::LocationMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[location]]></MsgType>\
        <Location_X>23.134521</Location_X>\
        <Location_Y>113.358803</Location_Y>
        <Scale>20</Scale>\
        <Label><![CDATA[位置信息]]></Label>
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = LocationMessage::from_xml(xml);

        assert_eq!("fromUser", msg.source());
        assert_eq!("toUser", msg.target());
        assert_eq!(1234567890123456, msg.id());
        assert_eq!(1348831860, msg.time());
        assert_eq!(23, msg.location_x() as usize);
        assert_eq!(113, msg.location_y() as usize);
        assert_eq!(20, msg.scale());
        assert_eq!("位置信息", msg.label());
    }
}
