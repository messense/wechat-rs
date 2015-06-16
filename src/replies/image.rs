use time;
use replies::ReplyRenderer;

pub struct ImageReply {
    pub source: String,
    pub target: String,
    pub time: i64,
    pub media_id: String,
}

impl ImageReply {
	pub fn new(source: &str, target: &str, media_id: &str) -> ImageReply {
		ImageReply {
			source: source.to_string(),
			target: target.to_string(),
			time: time::get_time().sec,
			media_id: media_id.to_string(),
		}
	}
}

impl ReplyRenderer for ImageReply {
	fn render(&self) -> String {
		format!("<xml>\n\
		    <ToUserName><![CDATA[{target}]]></ToUserName>\n\
		    <FromUserName><![CDATA[{source}]]></FromUserName>\n\
		    <CreateTime>{time}</CreateTime>\n\
		    <MsgType><![CDATA[image]]></MsgType>\n\
		    <Image>\n\
		    <MediaId><![CDATA[{media_id}]]></MediaId>\n\
		    </Image>\n\
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
	use super::ImageReply;

	#[test]
	fn test_render_text_reply() {
		let reply = ImageReply::new("test1", "test2", "test");
		let rendered = reply.render();
		assert!(rendered.contains("test1"));
		assert!(rendered.contains("test2"));
		assert!(rendered.contains("test"));
	}
}