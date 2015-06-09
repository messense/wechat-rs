use xmlutil;
use messages;
use messages::MessageParser;
use messages::Message;


pub fn parse_message(xml: &str) -> Message {
    let package = xmlutil::parse(xml);
    let doc = package.as_document();
    let msg_type_str = xmlutil::evaluate(&doc, "//xml/MsgType/text()").string().to_lowercase();
    let msg_type = &msg_type_str[..];
    let msg = match msg_type {
        "text" => Message::TextMessage(messages::TextMessage::from_xml(xml)),
        "image" => Message::ImageMessage(messages::ImageMessage::from_xml(xml)),
        "voice" => Message::VoiceMessage(messages::VoiceMessage::from_xml(xml)),
        "shortvideo" => Message::ShortVideoMessage(messages::ShortVideoMessage::from_xml(xml)),
        "video" => Message::VideoMessage(messages::VideoMessage::from_xml(xml)),
        "location" => Message::LocationMessage(messages::LocationMessage::from_xml(xml)),
        "link" => Message::LinkMessage(messages::LinkMessage::from_xml(xml)),
        "event" => {
            let event_str = xmlutil::evaluate(&doc, "//xml/Event/text()").string().to_lowercase();
            parse_event(&event_str[..], xml)
        },
        _ => Message::UnknownMessage(messages::UnknownMessage::from_xml(xml)),
    };
    msg
}

fn parse_event(event: &str, xml: &str) -> Message {
    match event {
        "subscribe" => Message::SubscribeEvent(messages::SubscribeEvent::from_xml(xml)),
        "unsubscribe" => Message::UnsubscribeEvent(messages::UnsubscribeEvent::from_xml(xml)),
        _ => Message::UnknownMessage(messages::UnknownMessage::from_xml(xml)),
    }
}

#[cfg(test)]
mod tests {
    use messages::Message;
    use super::parse_message;

    #[test]
    fn test_parse_text_message() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[text]]></MsgType>\
        <Content><![CDATA[this is a test]]></Content>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let _msg = parse_message(xml);
        let msg = match _msg {
            Message::TextMessage(ref m) => m,
            _ => panic!("Error parsing text message"),
        };

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("this is a test", &msg.content);
    }

    #[test]
    fn test_parse_image_message() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[image]]></MsgType>\
        <PicUrl><![CDATA[this is a url]]></PicUrl>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let _msg = parse_message(xml);
        let msg = match _msg {
            Message::ImageMessage(ref m) => m,
            _ => panic!("Error parsing image message"),
        };

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("media_id", &msg.media_id);
        assert_eq!("this is a url", &msg.image);
    }

    #[test]
    fn test_parse_voice_message() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[voice]]></MsgType>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <Format><![CDATA[Format]]></Format>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let _msg = parse_message(xml);
        let msg = match _msg {
            Message::VoiceMessage(ref m) => m,
            _ => panic!("Error parsing voice message"),
        };

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("media_id", &msg.media_id);
        assert_eq!("Format", &msg.format);
        assert_eq!("", &msg.recognition);
    }

    #[test]
    fn test_parse_video_message() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[video]]></MsgType>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let _msg = parse_message(xml);
        let msg = match _msg {
            Message::VideoMessage(ref m) => m,
            _ => panic!("Error parsing video message"),
        };

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("media_id", &msg.media_id);
        assert_eq!("thumb_media_id", &msg.thumb_media_id);
    }

    #[test]
    fn test_parse_shortvideo_message() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[shortvideo]]></MsgType>\
        <MediaId><![CDATA[media_id]]></MediaId>\
        <ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let _msg = parse_message(xml);
        let msg = match _msg {
            Message::ShortVideoMessage(ref m) => m,
            _ => panic!("Error parsing shortvideo message"),
        };

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("media_id", &msg.media_id);
        assert_eq!("thumb_media_id", &msg.thumb_media_id);
    }

    #[test]
    fn test_parse_link_message() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[link]]></MsgType>\
        <Title><![CDATA[公众平台官网链接]]></Title>\
        <Description><![CDATA[公众平台官网链接]]></Description>\
        <Url><![CDATA[url]]></Url>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let _msg = parse_message(xml);
        let msg = match _msg {
            Message::LinkMessage(ref m) => m,
            _ => panic!("Error parsing link message"),
        };

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!("公众平台官网链接", &msg.title);
        assert_eq!("公众平台官网链接", &msg.description);
        assert_eq!("url", &msg.url);
    }

    #[test]
    fn test_parse_location_message() {
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
        let _msg = parse_message(xml);
        let msg = match _msg {
            Message::LocationMessage(ref m) => m,
            _ => panic!("Error parsing location message"),
        };

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(1348831860, msg.time);
        assert_eq!(23, msg.location_x as usize);
        assert_eq!(113, msg.location_y as usize);
        assert_eq!(20, msg.scale);
        assert_eq!("位置信息", &msg.label);
    }

    #[test]
    fn test_parse_unknown_message() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[unknown_msg_type]]></MsgType>\
        <Content><![CDATA[this is a test]]></Content>\
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let _msg = parse_message(xml);
        match _msg {
            Message::UnknownMessage(_) => {},
            _ => panic!("Error parsing unknown message"),
        };
    }
}
