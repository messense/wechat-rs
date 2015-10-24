use std::io::Read;
use std::collections::HashMap;

use types::WeChatResult;
use client::WeChatClient;
use session::SessionStore;
use client::request::ArticleMaterial;
use client::response::{Material, MaterialCount, MaterialItem, MaterialItemList};


#[derive(Debug, Clone)]
pub struct WeChatMaterial<'a, T: SessionStore + 'a> {
    client: &'a WeChatClient<T>,
}

impl<'a, T: SessionStore> WeChatMaterial<'a, T> {
    #[inline]
    pub fn new(client: &'a WeChatClient<T>) -> WeChatMaterial<'a, T> {
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
        let data = json!({"articles": (articles)});
        let res = try!(self.client.post("material/add_news", vec![], &data));
        let media_id = &res["media_id"];
        let media_id = media_id.as_string().unwrap();
        Ok(Material {
            media_id: media_id.to_owned(),
            url: None,
        })
    }

    pub fn update_article<S: AsRef<str>>(&self, media_id: S, index: usize, article: ArticleMaterial) -> WeChatResult<()> {
        let data = json!({
            "media_id": (media_id.as_ref()),
            "index": (index),
            "articles": (article)
        });
        try!(self.client.post("material/update_news", vec![], &data));
        Ok(())
    }

    pub fn delete<S: AsRef<str>>(&self, media_id: S) -> WeChatResult<()> {
        let data = json!({"media_id": (media_id.as_ref())});
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

    pub fn get_list<S: AsRef<str>>(&self, media_type: S, offset: usize, count: usize) -> WeChatResult<MaterialItemList> {
        let data = json!({
            "type": (media_type.as_ref()),
            "offset": (offset),
            "count": (count),
        });
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
}
