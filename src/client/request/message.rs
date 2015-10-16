use rustc_serialize::json::{Json, ToJson};

#[derive(Debug, Clone)]
pub struct SendTextRequest {
    openid: String,
    account: Option<String>,
    content: String,
}

#[derive(Debug, Clone)]
pub struct SendImageRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
}

#[derive(Debug, Clone)]
pub struct SendVoiceRequest {
    openid: String,
    account: Option<String>,
    media_id: String,
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
