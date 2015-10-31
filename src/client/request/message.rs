use rustc_serialize::json::{Json, ToJson};
use jsonway;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendTextRequest {
    openid: String,
    account: Option<String>,
    content: String,
}

impl SendTextRequest {
    pub fn new<S: Into<String>>(openid: S, content: S) -> SendTextRequest {
        SendTextRequest {
            openid: openid.into(),
            content: content.into(),
            account: None,
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, content: S, account: S) -> SendTextRequest {
        SendTextRequest {
            openid: openid.into(),
            content: content.into(),
            account: Some(account.into()),
        }
    }
}

impl ToJson for SendTextRequest {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("msgtype", "text".to_owned());
            obj.set("touser", self.openid.to_owned());
            obj.object("text", |obj| {
                obj.set("content", self.content.to_owned());
            });
            if let Some(ref account) = self.account {
                obj.object("customservice", |obj| {
                    obj.set("kf_account", account.to_owned());
                });
            }
        }).unwrap()
    }
}

make_encodable!(SendTextRequest);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendImageRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
}

impl SendImageRequest {
    pub fn new<S: Into<String>>(openid: S, media_id: S) -> SendImageRequest {
        SendImageRequest {
            openid: openid.into(),
            account: None,
            media_id: media_id.into(),
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, media_id: S, account: S) -> SendImageRequest {
        SendImageRequest {
            openid: openid.into(),
            account: Some(account.into()),
            media_id: media_id.into(),
        }
    }
}
impl ToJson for SendImageRequest {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("msgtype", "image".to_owned());
            obj.set("touser", self.openid.to_owned());
            obj.object("image", |obj| {
                obj.set("media_id", self.media_id.to_owned());
            });
            if let Some(ref account) = self.account {
                obj.object("customservice", |obj| {
                    obj.set("kf_account", account.to_owned());
                });
            }
        }).unwrap()
    }
}


make_encodable!(SendImageRequest);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendVoiceRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
}

impl SendVoiceRequest {
    pub fn new<S: Into<String>>(openid: S, media_id: S) -> SendVoiceRequest {
        SendVoiceRequest {
            openid: openid.into(),
            account: None,
            media_id: media_id.into(),
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, media_id: S, account: S) -> SendVoiceRequest {
        SendVoiceRequest {
            openid: openid.into(),
            account: Some(account.into()),
            media_id: media_id.into(),
        }
    }
}

impl ToJson for SendVoiceRequest {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("msgtype", "voice".to_owned());
            obj.set("touser", self.openid.to_owned());
            obj.object("voice", |obj| {
                obj.set("media_id", self.media_id.to_owned());
            });
            if let Some(ref account) = self.account {
                obj.object("customservice", |obj| {
                    obj.set("kf_account", account.to_owned());
                });
            }
        }).unwrap()
    }
}

make_encodable!(SendVoiceRequest);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendVideoRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
    thumb_media_id: String,
    title: Option<String>,
    description: Option<String>,
}

impl SendVideoRequest {
    pub fn new<S: Into<String>>(openid: S, media_id: S, thumb_media_id: S, title: Option<String>, description: Option<String>) -> SendVideoRequest {
        SendVideoRequest {
            openid: openid.into(),
            account: None,
            media_id: media_id.into(),
            thumb_media_id: thumb_media_id.into(),
            title: title,
            description: description,
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, account: S, media_id: S, thumb_media_id: S, title: Option<String>, description: Option<String>) -> SendVideoRequest {
        SendVideoRequest {
            openid: openid.into(),
            account: Some(account.into()),
            media_id: media_id.into(),
            thumb_media_id: thumb_media_id.into(),
            title: title,
            description: description,
        }
    }
}

impl ToJson for SendVideoRequest {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("msgtype", "video".to_owned());
            obj.set("touser", self.openid.to_owned());
            obj.object("video", |obj| {
                obj.set("media_id", self.media_id.to_owned());
                obj.set("thumb_media_id", self.thumb_media_id.to_owned());
                obj.set("title", self.title.to_owned());
                obj.set("description", self.description.to_owned());
            });
            if let Some(ref account) = self.account {
                obj.object("customservice", |obj| {
                    obj.set("kf_account", account.to_owned());
                });
            }
        }).unwrap()
    }
}

make_encodable!(SendVideoRequest);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendMusicRequest {
    openid: String,
    account: Option<String>,
    music_url: String,
    hq_music_url: String,
    thumb_media_id: String,
    title: Option<String>,
    description: Option<String>,
}

impl SendMusicRequest {
    pub fn new<S: Into<String>>(openid: S, music_url: S, hq_music_url: S, thumb_media_id: S, title: Option<String>, description: Option<String>) -> SendMusicRequest {
        SendMusicRequest {
            openid: openid.into(),
            account: None,
            music_url: music_url.into(),
            hq_music_url: hq_music_url.into(),
            thumb_media_id: thumb_media_id.into(),
            title: title,
            description: description,
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, account: S, music_url: S, hq_music_url: S, thumb_media_id: S, title: Option<String>, description: Option<String>) -> SendMusicRequest {
        SendMusicRequest {
            openid: openid.into(),
            account: Some(account.into()),
            music_url: music_url.into(),
            hq_music_url: hq_music_url.into(),
            thumb_media_id: thumb_media_id.into(),
            title: title,
            description: description,
        }
    }
}

impl ToJson for SendMusicRequest {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("msgtype", "music".to_owned());
            obj.set("touser", self.openid.to_owned());
            obj.object("music", |obj| {
                obj.set("musicurl", self.music_url.to_owned());
                obj.set("hqmusicurl", self.hq_music_url.to_owned());
                obj.set("thumb_media_id", self.thumb_media_id.to_owned());
                obj.set("title", self.title.to_owned());
                obj.set("description", self.description.to_owned());
            });
            if let Some(ref account) = self.account {
                obj.object("customservice", |obj| {
                    obj.set("kf_account", account.to_owned());
                });
            }
        }).unwrap()
    }
}

make_encodable!(SendMusicRequest);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    title: String,
    url: String,
    description: Option<String>,
    image: Option<String>,
}

impl Article {
    pub fn new<S: Into<String>>(title: S, url: S, description: Option<String>, image: Option<String>) -> Article {
        Article {
            title: title.into(),
            url: url.into(),
            description: description,
            image: image,
        }
    }
}

impl ToJson for Article {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("title", self.title.to_owned());
            obj.set("url", self.url.to_owned());
            if let Some(ref desc) = self.description {
                obj.set("description", desc.to_owned());
            }
            if let Some(ref picurl) = self.image {
                obj.set("picurl", picurl.to_owned());
            }
        }).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendArticlesRequest {
    openid: String,
    account: Option<String>,
    articles: Vec<Article>,
}

impl SendArticlesRequest {
    pub fn new<S: Into<String>>(openid: S, articles: &[Article]) -> SendArticlesRequest {
        SendArticlesRequest {
            openid: openid.into(),
            account: None,
            articles: articles.to_vec(),
        }
    }

    pub fn with_account<S: Into<String>>(openid: S, account: S, articles: &[Article]) -> SendArticlesRequest {
        SendArticlesRequest {
            openid: openid.into(),
            account: Some(account.into()),
            articles: articles.to_vec(),
        }
    }

    pub fn add_article(&mut self, article: Article) {
        if self.articles.len() < 10 {
            self.articles.push(article);
        }
    }
}

impl ToJson for SendArticlesRequest {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("msgtype", "news".to_owned());
            obj.set("touser", self.openid.to_owned());
            obj.object("news", |obj| {
                obj.set("articles", self.articles.clone());
            });
            if let Some(ref account) = self.account {
                obj.object("customservice", |obj| {
                    obj.set("kf_account", account.to_owned());
                });
            }
        }).unwrap()
    }
}

make_encodable!(SendArticlesRequest);
