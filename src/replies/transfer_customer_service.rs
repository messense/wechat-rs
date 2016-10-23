use replies::ReplyRenderer;
use utils::current_timestamp;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TransferCustomerServiceReply {
    pub source: String,
    pub target: String,
    pub time: i64,
}

impl TransferCustomerServiceReply {
    #[inline]
    pub fn new<S: Into<String>>(source: S, target: S) -> TransferCustomerServiceReply {
        TransferCustomerServiceReply {
            source: source.into(),
            target: target.into(),
            time: current_timestamp(),
        }
    }
}

impl ReplyRenderer for TransferCustomerServiceReply {
    #[inline]
    fn render(&self) -> String {
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[transfer_customer_service]]></MsgType>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
        )
    }
}

#[cfg(test)]
mod tests {
    use replies::ReplyRenderer;
    use super::TransferCustomerServiceReply;

    #[test]
    fn test_render_transfer_customer_service_reply() {
        let reply = TransferCustomerServiceReply::new("test1", "test2");
        let rendered = reply.render();
        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("transfer_customer_service"));
    }
}
