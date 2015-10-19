#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KFAccount {
    pub id: String,
    pub nick: String,
    pub account: String,
    pub avatar: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnlineKFAccount {
    pub id: String,
    pub account: String,
    pub status: u64,
    pub auto_accept: u64,
    pub accepted_case: u64,
}
