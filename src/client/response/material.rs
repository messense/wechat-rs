#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Material {
    pub media_id: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialCount {
    pub voice_count: usize,
    pub video_count: usize,
    pub image_count: usize,
    pub articles_count: usize,
}
