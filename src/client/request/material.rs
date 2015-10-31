use rustc_serialize::json::{Json, ToJson};
use jsonway;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleMaterial {
    title: String,
    thumb_media_id: String,
    author: String,
    digest: String,
    show_cover_pic: u8,
    content: String,
    content_source_url: String,
}

impl ArticleMaterial {
    pub fn new<S: Into<String>>(title: S, thumb_media_id: S, author: S, digest: S, show_cover_pic: bool, content: S, content_source_url: S) -> ArticleMaterial {
        let show_cover = match show_cover_pic {
            true => 1,
            false => 0,
        };
        ArticleMaterial {
            title: title.into(),
            thumb_media_id: thumb_media_id.into(),
            author: author.into(),
            digest: digest.into(),
            show_cover_pic: show_cover,
            content: content.into(),
            content_source_url: content_source_url.into(),
        }
    }
}

impl ToJson for ArticleMaterial {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("title", self.title.to_owned());
            obj.set("thumb_media_id", self.thumb_media_id.to_owned());
            obj.set("author", self.author.to_owned());
            obj.set("digest", self.digest.to_owned());
            obj.set("show_cover_pic", self.show_cover_pic);
            obj.set("content", self.content.to_owned());
            obj.set("content_source_url", self.content_source_url.to_owned());
        }).unwrap()
    }
}

make_encodable!(ArticleMaterial);
