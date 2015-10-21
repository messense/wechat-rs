use std::io::Read;
use std::collections::HashMap;

use types::WeChatResult;
use client::WeChatClient;
use session::SessionStore;
use client::response::Media;


#[derive(Debug, Clone)]
pub struct WeChatMedia<'a, T: SessionStore + 'a> {
    client: &'a WeChatClient<T>,
}

impl<'a, T: SessionStore> WeChatMedia<'a, T> {
    #[inline]
    pub fn new(client: &'a WeChatClient<T>) -> WeChatMedia<'a, T> {
        WeChatMedia {
            client: client,
        }
    }

    pub fn upload<S: AsRef<str>, R: Read>(&self, media_type: S, media: &mut R) -> WeChatResult<Media> {
        let mut files = HashMap::new();
        files.insert("media".to_owned(), media);
        let res = try!(
            self.client.upload_file("media/upload", vec![("type", media_type.as_ref())], &mut files)
        );
        let media_id = &res["media_id"];
        let media_id = media_id.as_string().unwrap();
        let created_at = &res["created_at"];
        let created_at = created_at.as_u64().unwrap();
        let media_type_res = &res["type"];
        let media_type_res = media_type_res.as_string().unwrap();
        Ok(Media {
            media_type: media_type_res.to_owned(),
            media_id: media_id.to_owned(),
            created_at: created_at,
        })
    }
}
