extern crate wechat;

use wechat::Message;

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
    let _msg = Message::parse(xml);
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
    let _msg = Message::parse(xml);
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
    let _msg = Message::parse(xml);
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
    let _msg = Message::parse(xml);
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
    let _msg = Message::parse(xml);
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
    let _msg = Message::parse(xml);
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
    let _msg = Message::parse(xml);
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
    let _msg = Message::parse(xml);
    match _msg {
        Message::UnknownMessage(_) => {},
        _ => panic!("Error parsing unknown message"),
    };
}

#[test]
fn test_parse_subscribe_event() {
    let xml = "<xml>\
    <ToUserName><![CDATA[toUser]]></ToUserName>\
    <FromUserName><![CDATA[fromUser]]></FromUserName>\
    <CreateTime>123456789</CreateTime>\
    <MsgType><![CDATA[event]]></MsgType>\
    <Event><![CDATA[subscribe]]></Event>\
    </xml>";
    let _msg = Message::parse(xml);
    let msg = match _msg {
        Message::SubscribeEvent(ref m) => m,
        _ => panic!("Error parsing subscribe event"),
    };

    assert_eq!("fromUser", &msg.source);
    assert_eq!("toUser", &msg.target);
    assert_eq!("subscribe", &msg.event);
    assert_eq!(123456789, msg.time);
}

#[test]
fn test_parse_unsubscribe_event() {
    let xml = "<xml>\
    <ToUserName><![CDATA[toUser]]></ToUserName>\
    <FromUserName><![CDATA[fromUser]]></FromUserName>\
    <CreateTime>123456789</CreateTime>\
    <MsgType><![CDATA[event]]></MsgType>\
    <Event><![CDATA[unsubscribe]]></Event>\
    </xml>";
    let _msg = Message::parse(xml);
    let msg = match _msg {
        Message::UnsubscribeEvent(ref m) => m,
        _ => panic!("Error parsing unsubscribe event"),
    };

    assert_eq!("fromUser", &msg.source);
    assert_eq!("toUser", &msg.target);
    assert_eq!("unsubscribe", &msg.event);
    assert_eq!(123456789, msg.time);
}

#[test]
fn test_parse_scan_event() {
    let xml = "<xml>\
    <ToUserName><![CDATA[toUser]]></ToUserName>\
    <FromUserName><![CDATA[fromUser]]></FromUserName>\
    <CreateTime>123456789</CreateTime>\
    <MsgType><![CDATA[event]]></MsgType>\
    <Event><![CDATA[SCAN]]></Event>\
    <EventKey><![CDATA[SCENE_VALUE]]></EventKey>\
    <Ticket><![CDATA[TICKET]]></Ticket>\
    </xml>";
    let _msg = Message::parse(xml);
    let msg = match _msg {
        Message::ScanEvent(ref m) => m,
        _ => panic!("Error parsing scan event"),
    };

    assert_eq!("fromUser", &msg.source);
    assert_eq!("toUser", &msg.target);
    assert_eq!("scan", &msg.event);
    assert_eq!(123456789, msg.time);
    assert_eq!("SCENE_VALUE", &msg.scene_id);
    assert_eq!("TICKET", &msg.ticket);
}

#[test]
fn test_parse_location_event() {
    let xml = "<xml>\
    <ToUserName><![CDATA[toUser]]></ToUserName>\
    <FromUserName><![CDATA[fromUser]]></FromUserName>\
    <CreateTime>123456789</CreateTime>\
    <MsgType><![CDATA[event]]></MsgType>\
    <Event><![CDATA[LOCATION]]></Event>\
    <Latitude>23.137466</Latitude>\
    <Longitude>113.352425</Longitude>\
    <Precision>119.385040</Precision>\
    </xml>";
    let _msg = Message::parse(xml);
    let msg = match _msg {
        Message::LocationEvent(ref m) => m,
        _ => panic!("Error parsing location event"),
    };

    assert_eq!("fromUser", &msg.source);
    assert_eq!("toUser", &msg.target);
    assert_eq!("location", &msg.event);
    assert_eq!(123456789, msg.time);
    assert_eq!(23, msg.latitude as usize);
    assert_eq!(113, msg.longitude as usize);
    assert_eq!(119, msg.precision as usize);
}

#[test]
fn test_parse_click_event() {
    let xml = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[CLICK]]></Event>
    <EventKey><![CDATA[EVENTKEY]]></EventKey>
    </xml>";
    let _msg = Message::parse(xml);
    let msg = match _msg {
        Message::ClickEvent(ref m) => m,
        _ => panic!("Error parsing click event"),
    };

    assert_eq!("fromUser", &msg.source);
    assert_eq!("toUser", &msg.target);
    assert_eq!("click", &msg.event);
    assert_eq!(123456789, msg.time);
    assert_eq!("EVENTKEY", &msg.key);
}

#[test]
fn test_parse_view_event() {
    let xml = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[VIEW]]></Event>
    <EventKey><![CDATA[www.qq.com]]></EventKey>
    </xml>";
    let _msg = Message::parse(xml);
    let msg = match _msg {
        Message::ViewEvent(ref m) => m,
        _ => panic!("Error parsing view event"),
    };

    assert_eq!("fromUser", &msg.source);
    assert_eq!("toUser", &msg.target);
    assert_eq!("view", &msg.event);
    assert_eq!(123456789, msg.time);
    assert_eq!("www.qq.com", &msg.url);
}

#[test]
fn test_parse_subscribe_scan_event() {
    let xml = "<xml>\
    <ToUserName><![CDATA[toUser]]></ToUserName>\
    <FromUserName><![CDATA[fromUser]]></FromUserName>\
    <CreateTime>123456789</CreateTime>\
    <MsgType><![CDATA[event]]></MsgType>\
    <Event><![CDATA[subscribe]]></Event>\
    <EventKey><![CDATA[qrscene_123123]]></EventKey>\
    <Ticket><![CDATA[TICKET]]></Ticket>\
    </xml>";
    let _msg = Message::parse(xml);
    let msg = match _msg {
        Message::SubscribeScanEvent(ref m) => m,
        _ => panic!("Error parsing subscribe_scan event"),
    };

    assert_eq!("fromUser", &msg.source);
    assert_eq!("toUser", &msg.target);
    assert_eq!("subscribe_scan", &msg.event);
    assert_eq!(123456789, msg.time);
    assert_eq!("123123", &msg.scene_id);
    assert_eq!("TICKET", &msg.ticket);
}
