use super::xmlutil;
use super::messages;
use super::messages::MessageParser;
use super::messages::Message;


pub fn parse_message(xml: &str) -> Message {
    let package = xmlutil::parse(xml);
    let doc = package.as_document();
    let msg_type_str = xmlutil::evaluate(&doc, "//xml/MsgType/text()").string();
    let msg_type = &msg_type_str[..];
    let msg = match msg_type {
        "text" => Message::Text(messages::TextMessage::from_xml(xml)),
        _ => Message::Unknown(messages::UnknownMessage::from_xml(xml)),
    };
    msg
}
