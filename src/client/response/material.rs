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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialItem {
    pub media_id: String,
    pub name: String,
    pub url: String,
    pub update_time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialItemList {
    pub total_count: u64,
    pub item_count: u64,
    pub items: Vec<MaterialItem>,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialArticle {
    pub title: String,
    pub thumb_media_id: String,
    pub author: String,
    pub digest: String,
    pub show_cover_pic: bool,
    pub content: String,
    pub url: String,
    pub content_source_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialArticleItem {
    pub media_id: String,
    pub articles: Vec<MaterialArticle>,
    pub update_time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialArticleList {
    pub total_count: u64,
    pub item_count: u64,
    pub items: Vec<MaterialArticleItem>,
}
