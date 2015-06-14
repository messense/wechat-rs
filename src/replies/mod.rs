pub trait ReplyRenderer {
	fn render(&self) -> String;
}

mod text;

pub use self::text::TextReply;

pub enum Reply {
    TextReply(TextReply),
}