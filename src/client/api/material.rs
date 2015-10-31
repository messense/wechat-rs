use std::io::Read;
use std::collections::HashMap;

use jsonway;

use types::WeChatResult;
use client::APIClient;
use session::SessionStore;
use client::request::ArticleMaterial;
use client::response::{
    Material,
    MaterialCount,
    MaterialItem,
    MaterialItemList,
    MaterialArticle,
    MaterialArticleItem,
    MaterialArticleList,
};

#[derive(Debug, Clone)]
pub struct WeChatMaterial<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatMaterial<T> {
    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatMaterial<T> {
        WeChatMaterial {
            client: client,
        }
    }

    pub fn add<S: AsRef<str>, R: Read>(&self, media_type: S, media: &mut R) -> WeChatResult<Material> {
        let mut files = HashMap::new();
        files.insert("media".to_owned(), media);
        let res = try!(
            self.client.upload_file("material/add_material", vec![("type", media_type.as_ref())], &mut files)
        );
        let media_id = &res["media_id"];
        let media_id = media_id.as_string().unwrap();
        let url = res.find("url").map(|x| x.as_string().unwrap().to_owned());
        Ok(Material {
            media_id: media_id.to_owned(),
            url: url,
        })
    }

    pub fn add_articles(&self, articles: &[ArticleMaterial]) -> WeChatResult<Material> {
        let data = jsonway::object(|obj| {
            obj.set("articles", articles.to_vec());
        }).unwrap();
        let res = try!(self.client.post("material/add_news", vec![], &data));
        let media_id = &res["media_id"];
        let media_id = media_id.as_string().unwrap();
        Ok(Material {
            media_id: media_id.to_owned(),
            url: None,
        })
    }

    pub fn update_article<S: Into<String>>(&self, media_id: S, index: usize, article: ArticleMaterial) -> WeChatResult<()> {
        let data = jsonway::object(|obj| {
            obj.set("media_id", media_id.into());
            obj.set("index", index);
            obj.set("articles", article);
        }).unwrap();
        try!(self.client.post("material/update_news", vec![], &data));
        Ok(())
    }

    pub fn delete<S: Into<String>>(&self, media_id: S) -> WeChatResult<()> {
        let data = jsonway::object(|obj| {
            obj.set("media_id", media_id.into());
        }).unwrap();
        try!(self.client.post("material/del_material", vec![], &data));
        Ok(())
    }

    pub fn get_count(&self) -> WeChatResult<MaterialCount> {
        let res = try!(self.client.get("material/get_materialcount", vec![]));
        let voice_count = &res["voice_count"];
        let voice_count = voice_count.as_u64().unwrap();
        let video_count = &res["video_count"];
        let video_count = video_count.as_u64().unwrap();
        let image_count = &res["image_count"];
        let image_count = image_count.as_u64().unwrap();
        let articles_count = &res["news_count"];
        let articles_count = articles_count.as_u64().unwrap();
        Ok(MaterialCount {
            voice_count: voice_count as usize,
            video_count: video_count as usize,
            image_count: image_count as usize,
            articles_count: articles_count as usize,
        })
    }

    pub fn get_list<S: Into<String>>(&self, media_type: S, offset: usize, count: usize) -> WeChatResult<MaterialItemList> {
        let data = jsonway::object(|obj| {
            obj.set("type", media_type.into());
            obj.set("offset", offset);
            obj.set("count", count);
        }).unwrap();
        let res = try!(self.client.post("material/batchget_material", vec![], &data));
        let total_count = &res["total_count"];
        let total_count = total_count.as_u64().unwrap();
        let item_count = &res["item_count"];
        let item_count = item_count.as_u64().unwrap();
        let mut items = vec![];
        match res.find("item") {
            Some(items_json) => {
                let items_json = items_json.as_array().unwrap();
                for item_json in items_json.iter() {
                    let media_id = &item_json["media_id"];
                    let media_id = media_id.as_string().unwrap();
                    let name = &item_json["name"];
                    let name = name.as_string().unwrap();
                    let url = &item_json["url"];
                    let url = url.as_string().unwrap();
                    let update_time = &item_json["update_time"];
                    let update_time = update_time.as_u64().unwrap();
                    items.push(MaterialItem {
                        media_id: media_id.to_owned(),
                        name: name.to_owned(),
                        url: url.to_owned(),
                        update_time: update_time,
                    });
                }
            },
            None => {},
        };
        Ok(MaterialItemList {
            total_count: total_count,
            item_count: item_count,
            items: items,
        })
    }

    pub fn get_article_list(&self, offset: usize, count: usize) -> WeChatResult<MaterialArticleList> {
        let data = jsonway::object(|obj| {
            obj.set("type", "news".to_owned());
            obj.set("offset", offset);
            obj.set("count", count);
        }).unwrap();
        let res = try!(self.client.post("material/batchget_material", vec![], &data));
        let total_count = &res["total_count"];
        let total_count = total_count.as_u64().unwrap();
        let item_count = &res["item_count"];
        let item_count = item_count.as_u64().unwrap();
        let mut items = vec![];
        match res.find("item") {
            Some(items_json) => {
                let items_json = items_json.as_array().unwrap();
                for item_json in items_json.iter() {
                    let media_id = &item_json["media_id"];
                    let media_id = media_id.as_string().unwrap();
                    let update_time = &item_json["update_time"];
                    let update_time = update_time.as_u64().unwrap();
                    let content_array = &item_json["content"];
                    let content_array = content_array.as_array().unwrap();
                    let mut contents = vec![];
                    for content_json in content_array.iter() {
                        let title = &content_json["title"];
                        let title = title.as_string().unwrap();
                        let thumb_media_id = &content_json["thumb_media_id"];
                        let thumb_media_id = thumb_media_id.as_string().unwrap();
                        let author = &content_json["author"];
                        let author = author.as_string().unwrap();
                        let digest = &content_json["digest"];
                        let digest = digest.as_string().unwrap();
                        let show_cover_pic = &content_json["show_cover_pic"];
                        let show_cover_pic = show_cover_pic.as_u64().unwrap();
                        let show_cover = match show_cover_pic {
                            1u64 => true,
                            _ => false,
                        };
                        let content = &content_json["content"];
                        let content = content.as_string().unwrap();
                        let url = &content_json["url"];
                        let url = url.as_string().unwrap();
                        let source_url = &content_json["content_source_url"];
                        let source_url = source_url.as_string().unwrap();
                        contents.push(MaterialArticle {
                            title: title.to_owned(),
                            thumb_media_id: thumb_media_id.to_owned(),
                            author: author.to_owned(),
                            digest: digest.to_owned(),
                            show_cover_pic: show_cover,
                            content: content.to_owned(),
                            url: url.to_owned(),
                            content_source_url: source_url.to_owned(),
                        });
                    }
                    items.push(MaterialArticleItem {
                        media_id: media_id.to_owned(),
                        articles: contents,
                        update_time: update_time,
                    });
                }
            },
            None => {},
        };
        Ok(MaterialArticleList {
            total_count: total_count,
            item_count: item_count,
            items: items,
        })
    }
}
