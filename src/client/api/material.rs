use std::io::Read;
use std::collections::HashMap;

use types::WeChatResult;
use client::WeChatClient;
use session::SessionStore;
use client::response::Material;


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
            self.client.upload_file("media/upload", vec![("type", media_type.as_ref())], &mut files)
        );
        let media_id = &res["media_id"];
        let media_id = media_id.as_string().unwrap();
        let url = res.find("url").map(|x| x.as_string().unwrap().to_owned());
        Ok(Material {
            media_id: media_id.to_owned(),
            url: url,
        })
    }
}
