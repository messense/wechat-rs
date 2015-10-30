use std::io::{Read, Seek, SeekFrom, Cursor};
use std::collections::HashMap;

use hyper::method::Method;
use rustc_serialize::json::{Json, Object};

use types::WeChatResult;
use errors::WeChatError;
use client::APIClient;
use session::SessionStore;
use client::response::Media;


#[derive(Debug, Clone)]
pub struct WeChatMedia<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatMedia<T> {
    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatMedia<T> {
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

    pub fn upload_image<R: Read>(&self, media:&mut R) -> WeChatResult<String> {
        let mut files = HashMap::new();
        files.insert("media".to_owned(), media);
        let res = try!(
            self.client.upload_file("media/uploadimg", vec![], &mut files)
        );
        let url = &res["url"];
        let url = url.as_string().unwrap();
        Ok(url.to_owned())
    }

    pub fn get<S: AsRef<str>>(&self, media_id: S) -> WeChatResult<Cursor<Vec<u8>>> {
        let mut res = try!(
            self.client.request(
                Method::Get,
                "media/get",
                vec![("media_id", media_id.as_ref())],
                &Object::new()
            )
        );
        let mut buff = vec![];
        try!(res.read_to_end(&mut buff));
        let mut cursor = Cursor::new(buff);
        match Json::from_reader(&mut cursor) {
            Ok(obj) => {
                match obj.find("errcode") {
                    Some(code) => {
                        let errcode = code.as_i64().unwrap();
                        let errmsg = match obj.find("errmsg") {
                            Some(msg) => {
                                msg.as_string().unwrap()
                            },
                            None => { "" }
                        };
                        return Err(WeChatError::ClientError {
                            errcode: errcode as i32,
                            errmsg: errmsg.to_owned()
                        });
                    },
                    None => {},
                }
            },
            Err(_) => {}
        }
        try!(cursor.seek(SeekFrom::Start(0u64)));
        Ok(cursor)
    }
}
