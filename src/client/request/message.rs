use rustc_serialize::json::{Json, ToJson};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendTextRequest {
    openid: String,
    account: Option<String>,
    content: String,
}

impl SendTextRequest {
    pub fn new(openid: &str, content: &str) -> SendTextRequest {
        SendTextRequest {
            openid: openid.to_owned(),
            content: content.to_owned(),
            account: None,
        }
    }

    pub fn with_account(openid: &str, content: &str, account: &str) -> SendTextRequest {
        SendTextRequest {
            openid: openid.to_owned(),
            content: content.to_owned(),
            account: Some(account.to_owned()),
        }
    }
}

impl ToJson for SendTextRequest {
    fn to_json(&self) -> Json {
        if let Some(ref account) = self.account {
            json!({
                "msgtype": "text",
                "touser": (self.openid),
                "text": {"content": (self.content)},
                "customservice": {"kf_account": (account)},
            })
        } else {
            json!({
                "msgtype": "text",
                "touser": (self.openid),
                "text": {"content": (self.content)},
            })
        }
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
    pub fn new(openid: &str, media_id: &str) -> SendImageRequest {
        SendImageRequest {
            openid: openid.to_owned(),
            account: None,
            media_id: media_id.to_owned(),
        }
    }

    pub fn with_account(openid: &str, media_id: &str, account: &str) -> SendImageRequest {
        SendImageRequest {
            openid: openid.to_owned(),
            account: Some(account.to_owned()),
            media_id: media_id.to_owned(),
        }
    }
}
impl ToJson for SendImageRequest {
    fn to_json(&self) -> Json {
        if let Some(ref account) = self.account {
            json!({
                "msgtype": "image",
                "touser": (self.openid),
                "image": {"media_id": (self.media_id)},
                "customservice": {"kf_account": (account)},
            })
        } else {
            json!({
                "msgtype": "image",
                "touser": (self.openid),
                "image": {"media_id": (self.media_id)},
            })
        }
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
    pub fn new(openid: &str, media_id: &str) -> SendVoiceRequest {
        SendVoiceRequest {
            openid: openid.to_owned(),
            account: None,
            media_id: media_id.to_owned(),
        }
    }

    pub fn with_account(openid: &str, media_id: &str, account: &str) -> SendVoiceRequest {
        SendVoiceRequest {
            openid: openid.to_owned(),
            account: Some(account.to_owned()),
            media_id: media_id.to_owned(),
        }
    }
}

impl ToJson for SendVoiceRequest {
    fn to_json(&self) -> Json {
        if let Some(ref account) = self.account {
            json!({
                "msgtype": "voice",
                "touser": (self.openid),
                "voice": {"media_id": (self.media_id)},
                "customservice": {"kf_account": (account)},
            })
        } else {
            json!({
                "msgtype": "voice",
                "touser": (self.openid),
                "voice": {"media_id": (self.media_id)},
            })
        }
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
    pub fn new(openid: &str, media_id: &str, thumb_media_id: &str, title: Option<String>, description: Option<String>) -> SendVideoRequest {
        SendVideoRequest {
            openid: openid.to_owned(),
            account: None,
            media_id: media_id.to_owned(),
            thumb_media_id: thumb_media_id.to_owned(),
            title: title,
            description: description,
        }
    }

    pub fn with_account(openid: &str, account: &str, media_id: &str, thumb_media_id: &str, title: Option<String>, description: Option<String>) -> SendVideoRequest {
        SendVideoRequest {
            openid: openid.to_owned(),
            account: Some(account.to_owned()),
            media_id: media_id.to_owned(),
            thumb_media_id: thumb_media_id.to_owned(),
            title: title,
            description: description,
        }
    }
}

impl ToJson for SendVideoRequest {
    fn to_json(&self) -> Json {
        if let Some(ref account) = self.account {
            json!({
                "msgtype": "video",
                "touser": (self.openid),
                "video": {
                    "media_id": (self.media_id),
                    "thumb_media_id": (self.thumb_media_id),
                    "title": (self.title),
                    "description": (self.description),
                },
                "customservice": {"kf_account": (account)},
            })
        } else {
            json!({
                "msgtype": "video",
                "touser": (self.openid),
                "video": {
                    "media_id": (self.media_id),
                    "thumb_media_id": (self.thumb_media_id),
                    "title": (self.title),
                    "description": (self.description),
                },
            })
        }
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
    pub fn new(openid: &str, music_url: &str, hq_music_url: &str, thumb_media_id: &str, title: Option<String>, description: Option<String>) -> SendMusicRequest {
        SendMusicRequest {
            openid: openid.to_owned(),
            account: None,
            music_url: music_url.to_owned(),
            hq_music_url: hq_music_url.to_owned(),
            thumb_media_id: thumb_media_id.to_owned(),
            title: title,
            description: description,
        }
    }

    pub fn with_account(openid: &str, account: &str, music_url: &str, hq_music_url: &str, thumb_media_id: &str, title: Option<String>, description: Option<String>) -> SendMusicRequest {
        SendMusicRequest {
            openid: openid.to_owned(),
            account: Some(account.to_owned()),
            music_url: music_url.to_owned(),
            hq_music_url: hq_music_url.to_owned(),
            thumb_media_id: thumb_media_id.to_owned(),
            title: title,
            description: description,
        }
    }
}

impl ToJson for SendMusicRequest {
    fn to_json(&self) -> Json {
        if let Some(ref account) = self.account {
            json!({
                "msgtype": "music",
                "touser": (self.openid),
                "music": {
                    "musicurl": (self.music_url),
                    "hqmusicurl": (self.hq_music_url),
                    "thumb_media_id": (self.thumb_media_id),
                    "title": (self.title),
                    "description": (self.description),
                },
                "customservice": {"kf_account": (account)},
            })
        } else {
            json!({
                "msgtype": "music",
                "touser": (self.openid),
                "music": {
                    "musicurl": (self.music_url),
                    "hqmusicurl": (self.hq_music_url),
                    "thumb_media_id": (self.thumb_media_id),
                    "title": (self.title),
                    "description": (self.description),
                },
            })
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendArticlesRequest {
    openid: String,
    account: Option<String>,
    articles: Vec<Article>,
}
