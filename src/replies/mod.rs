pub trait ReplyRenderer {
	fn render(&self) -> String;
}

mod text;
mod image;

pub use self::text::TextReply;
pub use self::image::ImageReply;

pub enum Reply {
    TextReply(TextReply),
    ImageReply(ImageReply),
}