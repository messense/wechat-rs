#[derive(Debug, Clone, PartialEq, Eq)]
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Followers {
    pub total: u64,
    pub count: u64,
    pub openids: Vec<String>,
    pub next_openid: String,
}
