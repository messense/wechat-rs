use rustc_serialize::json::{Json, ToJson};


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
        json!({
            "title": (self.title),
            "thumb_media_id": (self.thumb_media_id),
            "author": (self.author),
            "digest": (self.digest),
            "show_cover_pic": (self.show_cover_pic),
            "content": (self.content),
            "content_source_url": (self.content_source_url),
        })
    }
}

make_encodable!(ArticleMaterial);
