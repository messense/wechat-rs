use replies::ReplyRenderer;
use utils::current_timestamp;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VoiceReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub media_id: String,
}

impl VoiceReply {
    #[inline]
    pub fn new<S: Into<String>>(source: S, target: S, media_id: S) -> VoiceReply {
        VoiceReply {
            source: source.into(),
            target: target.into(),
            time: current_timestamp(),
            media_id: media_id.into(),
        }
    }
}

impl ReplyRenderer for VoiceReply {
    #[inline]
    fn render(&self) -> String {
        format!("<xml>\n\
            <ToUserName><![CDATA[{target}]]></ToUserName>\n\
            <FromUserName><![CDATA[{source}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[voice]]></MsgType>\n\
            <Voice>\n\
            <MediaId><![CDATA[{media_id}]]></MediaId>\n\
            </Voice>\n\
            </xml>",
            target=self.target,
            source=self.source,
            time=self.time,
            media_id=self.media_id
        )
    }
}

#[cfg(test)]
mod tests {
    use replies::ReplyRenderer;
    use super::VoiceReply;

    #[test]
    fn test_render_voice_reply() {
        let reply = VoiceReply::new("test1", "test2", "test");
        let rendered = reply.render();
        assert!(rendered.contains("test1"));
        assert!(rendered.contains("test2"));
        assert!(rendered.contains("test"));
    }
}
