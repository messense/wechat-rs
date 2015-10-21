#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Media {
    pub media_type: String,
    pub media_id: String,
    pub created_at: u64,
}
