use std::io::Read;
use std::collections::HashMap;

use url::Url;
use hyper::{self, Client};
use hyper::client::{Request, Response};
use hyper::method::Method;
use rustc_serialize::json::{self, Json, Object};
use rustc_serialize::Encodable;
use multipart::client::Multipart;

use errors::WeChatError;
use types::WeChatResult;
use session::SessionStore;
use utils::current_timestamp;


const REFETCH_ACCESS_TOKEN_ERRCODES: [i32; 3] = [40001, 40014, 42001];


#[derive(Debug, Clone)]
pub struct APIClient<T: SessionStore> {
    pub appid: String,
    pub secret: String,
    pub session: T,
}

impl<T: SessionStore> APIClient<T> {

    #[inline]
    pub fn new<S: Into<String>>(appid: S, secret: S, session: T) -> APIClient<T> {
        APIClient {
            appid: appid.into(),
            secret: secret.into(),
            session: session,
        }
    }

    #[inline]
    pub fn with_access_token<S: Into<String>>(appid: S, secret: S, access_token: S, session: T) -> APIClient<T> {
        let appid = appid.into();
        let secret = secret.into();
        let client = Self::new(appid.clone(), secret, session);
        let token_key = format!("{}_access_token", appid);
        client.session.set(&token_key, access_token.into(), None);
        client
    }

    #[inline]
    pub fn access_token(&self) -> String {
        let token_key = format!("{}_access_token", self.appid);
        let expires_key = format!("{}_expires_at", self.appid);
        let token: String = self.session.get(&token_key, Some("".to_owned())).unwrap();
        let timestamp = current_timestamp();
        let expires_at: i64 = self.session.get(&expires_key, Some(timestamp)).unwrap();
        if expires_at <= timestamp {
            "".to_owned()
        } else {
            token
        }
    }

    pub fn request<D: Encodable>(&self, method: Method, url: &str, params: Vec<(&str, &str)>, data: &D) -> WeChatResult<Response> {
        let mut http_url = if !(url.starts_with("http://") || url.starts_with("https://")) {
            let mut url_string = "https://api.weixin.qq.com/cgi-bin/".to_owned();
            url_string = url_string + url;
            Url::parse(&url_string).unwrap()
        } else {
            Url::parse(url).unwrap()
        };
        let access_token = self.access_token();
        let mut querys = params.clone();
        if !access_token.is_empty() {
            debug!("Using access_token: {}", access_token);
            querys.push(("access_token", &access_token));
        }
        http_url.query_pairs_mut().extend_pairs(querys.into_iter());
        let body = match json::encode(data) {
            Ok(text) => text,
            Err(_) => "".to_owned(),
        };
        let client = Client::new();
        let req = if method == Method::Post {
            client.post(http_url).body(&body)
        } else {
            client.get(http_url)
        };
        let res = match req.send() {
            Ok(_res) => _res,
            Err(ref e) => {
                error!("Send request error");
                return Err(WeChatError::ClientError { errcode: -1, errmsg: format!("Send request error: {}", e) });
            }
        };
        if res.status != hyper::Ok {
            error!("Response status error");
            return Err(WeChatError::ClientError { errcode: -2, errmsg: "Response status error".to_owned() })
        }
        Ok(res)

    }

    fn _upload_file<R: Read>(&self, url: &str, params: Vec<(&str, &str)>, files: &mut HashMap<String, &mut R>) -> WeChatResult<Response> {
        let mut http_url = if !(url.starts_with("http://") || url.starts_with("https://")) {
            let mut url_string = "https://api.weixin.qq.com/cgi-bin/".to_owned();
            url_string = url_string + url;
            Url::parse(&url_string).unwrap()
        } else {
            Url::parse(url).unwrap()
        };
        let access_token = self.access_token();
        let mut querys = params.clone();
        if !access_token.is_empty() {
            debug!("Using access_token: {}", access_token);
            querys.push(("access_token", &access_token));
        }
        http_url.query_pairs_mut().extend_pairs(querys.into_iter());
        let request = Request::new(Method::Post, http_url).unwrap();
        let mut req = Multipart::from_request(request).unwrap();
        for (name, stream) in files.iter_mut() {
            // TODO: error handing
            req.write_stream(name, stream, None, None).unwrap();
        }
        let res = match req.send() {
            Ok(_res) => _res,
            Err(ref e) => {
                error!("Send request error");
                return Err(WeChatError::ClientError { errcode: -1, errmsg: format!("Send request error: {}", e) });
            }
        };
        if res.status != hyper::Ok {
            error!("Response status error");
            return Err(WeChatError::ClientError { errcode: -2, errmsg: "Response status error".to_owned() })
        }
        Ok(res)
    }

    pub fn upload_file<R: Read>(&self, url: &str, params: Vec<(&str, &str)>, files: &mut HashMap<String, &mut R>) -> WeChatResult<Json> {
        if self.access_token().is_empty() {
            self.fetch_access_token();
        }
        let mut res = try!(self._upload_file(url, params.clone(), files));
        let data = match self.json_decode(&mut res) {
            Ok(_data) => _data,
            Err(err) => {
                if let WeChatError::ClientError { errcode, .. } = err {
                    if REFETCH_ACCESS_TOKEN_ERRCODES.contains(&errcode) {
                        // access_token expired, fetch a new one and retry request
                        self.fetch_access_token();
                        let mut res1 = try!(self._upload_file(url, params, files));
                        try!(self.json_decode(&mut res1))
                    } else {
                        return Err(err);
                    }
                } else {
                    return Err(err);
                }
            },
        };
        Ok(data)
    }

    #[inline]
    fn json_decode(&self, res: &mut Response) -> WeChatResult<Json> {
        let obj = match Json::from_reader(res) {
            Ok(decoded) => { decoded },
            Err(ref e) => {
                error!("Json decode error");
                return Err(WeChatError::ClientError { errcode: -3, errmsg: format!("Json decode error: {}", e) });
            }
        };
        match obj.find("errcode") {
            Some(code) => {
                let errcode = code.as_i64().unwrap();
                if errcode != 0 {
                    let errmsg = match obj.find("errmsg") {
                        Some(msg) => {
                            msg.as_string().unwrap()
                        },
                        None => { "" }
                    };
                    return Err(WeChatError::ClientError { errcode: errcode as i32, errmsg: errmsg.to_owned() });
                }
            },
            None => {},
        }
        Ok(obj)
    }

    #[inline]
    pub fn post<D: Encodable>(&self, url: &str, params: Vec<(&str, &str)>, data: &D) -> WeChatResult<Json> {
        if self.access_token().is_empty() {
            self.fetch_access_token();
        }
        let mut res = try!(self.request(Method::Post, url, params.clone(), data));
        let data = match self.json_decode(&mut res) {
            Ok(_data) => _data,
            Err(err) => {
                if let WeChatError::ClientError { errcode, .. } = err {
                    if REFETCH_ACCESS_TOKEN_ERRCODES.contains(&errcode) {
                        // access_token expired, fetch a new one and retry request
                        self.fetch_access_token();
                        let mut res1 = try!(self.request(Method::Post, url, params, data));
                        try!(self.json_decode(&mut res1))
                    } else {
                        return Err(err);
                    }
                } else {
                    return Err(err);
                }
            },
        };
        Ok(data)
    }

    #[inline]
    pub fn get(&self, url: &str, params: Vec<(&str, &str)>) -> WeChatResult<Json> {
        if self.access_token().is_empty() {
            self.fetch_access_token();
        }
        let mut res = try!(self.request(Method::Get, url, params.clone(), &Object::new()));
        let data = match self.json_decode(&mut res) {
            Ok(_data) => _data,
            Err(err) => {
                if let WeChatError::ClientError { errcode, .. } = err {
                    if REFETCH_ACCESS_TOKEN_ERRCODES.contains(&errcode) {
                        // access_token expired, fetch a new one and retry request
                        self.fetch_access_token();
                        let mut res1 = try!(self.request(Method::Get, url, params, &Object::new()));
                        try!(self.json_decode(&mut res1))
                    } else {
                        return Err(err);
                    }
                } else {
                    return Err(err);
                }
            },
        };
        Ok(data)
    }

    pub fn fetch_access_token(&self) -> Option<String> {
        let res = self.request(
            Method::Get,
            "token",
            vec![
                ("grant_type", "client_credential"),
                ("appid", &self.appid),
                ("secret", &self.secret),
            ],
            &Object::new()
        );
        let mut raw_data = match res {
            Ok(raw) => raw,
            Err(_) => { return None; },
        };
        let data = match self.json_decode(&mut raw_data) {
            Ok(data) => data,
            Err(_) => { return None; },
        };
        let token = match data.find("access_token") {
            Some(token) => token,
            None => { return None; }
        };
        let expires_in = match data.find("expires_in") {
            Some(expires) => expires.as_u64().unwrap() as usize,
            None => 7200usize,
        };
        let expires_at = current_timestamp() + expires_in as i64;
        let token_str = match *token {
            Json::String(ref v) => {
                let token_key = format!("{}_access_token", self.appid);
                let expires_key = format!("{}_expires_at", self.appid);
                self.session.set(&token_key, v.to_owned(), Some(expires_in));
                self.session.set(&expires_key, expires_at, Some(expires_in));
                Some(format!("{}", v))
            },
            _ => None,
        };
        token_str
    }
}
