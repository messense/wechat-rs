#[derive(Debug, Clone)]
pub struct User {
    pub subscribe: bool,
    pub openid: String,
    pub nickname: String,
    pub sex: u8,
    pub language: String,
    pub city: String,
    pub province: String,
    pub country: String,
    pub avatar: String,
    pub subscribe_time: u64,
    pub unionid: Option<String>,
    pub remark: String,
    pub group_id: u64,
}
