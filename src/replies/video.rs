use time;
use replies::ReplyRenderer;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VideoReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub media_id: String,
    pub title: String,
    pub description: String,
}

impl VideoReply {
	#[inline]
	pub fn new(source: &str, target: &str, media_id: &str) -> VideoReply {
		VideoReply {
			source: source.to_owned(),
			target: target.to_owned(),
			time: time::get_time().sec,
			media_id: media_id.to_owned(),
			title: "".to_owned(),
			description: "".to_owned(),
		}
	}
}

impl ReplyRenderer for VideoReply {
	#[inline]
	fn render(&self) -> String {
		format!("<xml>\n\
		    <ToUserName><![CDATA[{target}]]></ToUserName>\n\
		    <FromUserName><![CDATA[{source}]]></FromUserName>\n\
		    <CreateTime>{time}</CreateTime>\n\
		    <MsgType><![CDATA[video]]></MsgType>\n\
		    <Video>\n\
		    <MediaId><![CDATA[{media_id}]]></MediaId>\n\
		    <Title><![CDATA[{title}]]></Title>\n\
		    <Description><![CDATA[{description}]]></Description>\n\
		    </Video>\n\
		    </xml>",
		    target=self.target,
		    source=self.source,
		    time=self.time,
		    media_id=self.media_id,
		    title=self.title,
		    description=self.description,
	    )
	}
}

#[cfg(test)]
mod tests {
	use replies::ReplyRenderer;
	use super::VideoReply;

	#[test]
	fn test_render_text_reply() {
		let reply = VideoReply::new("test1", "test2", "test");
		let rendered = reply.render();
		assert!(rendered.contains("test1"));
		assert!(rendered.contains("test2"));
		assert!(rendered.contains("test"));
	}
}