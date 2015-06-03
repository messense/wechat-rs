use super::super::xmlutil;
use super::{MessageParser, MessageData};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LinkMessage {
    source: String,
    target: String,
    time: usize,
    id: usize,
    title: String,
    description: String,
    url: String,
}

impl MessageData for LinkMessage {
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

impl MessageParser for LinkMessage {
    type WeChatMessage = LinkMessage;

    fn from_xml(xml: &str) -> LinkMessage {
        let package = xmlutil::parse(xml);
        let doc = package.as_document();
        let source = xmlutil::evaluate(&doc, "//xml/FromUserName/text()");
        let target = xmlutil::evaluate(&doc, "//xml/ToUserName/text()");
        let id = xmlutil::evaluate(&doc, "//xml/MsgId/text()");
        let time = xmlutil::evaluate(&doc, "//xml/CreateTime/text()");
        let title = xmlutil::evaluate(&doc, "//xml/Title/text()");
        let description = xmlutil::evaluate(&doc, "//xml/Description/text()");
        let url = xmlutil::evaluate(&doc, "//xml/Url/text()");
        LinkMessage {
            source: source.string(),
            target: target.string(),
            id: id.number() as usize,
            time: time.number() as usize,
            title: title.string(),
            description: description.string(),
            url: url.string(),
        }
    }
}

impl LinkMessage {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use messages::{MessageParser, MessageData};
    use super::LinkMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[text]]></MsgType>\
        <Title><![CDATA[公众平台官网链接]]></Title>\
        <Description><![CDATA[公众平台官网链接]]></Description>\
        <Url><![CDATA[url]]></Url>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = LinkMessage::from_xml(xml);

        assert_eq!("fromUser", msg.source());
        assert_eq!("toUser", msg.target());
        assert_eq!(1234567890123456, msg.id());
        assert_eq!(1348831860, msg.time());
        assert_eq!("公众平台官网链接", msg.title());
        assert_eq!("公众平台官网链接", msg.description());
        assert_eq!("url", msg.url());
    }
}
