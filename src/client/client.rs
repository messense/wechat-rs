use url::Url;
use hyper::{self, Client};
use hyper::client::Response;
use hyper::method::Method;
use rustc_serialize::json::{self, Json, Object};
use rustc_serialize::Encodable;

use errors::WeChatError;
use client::WeChatResult;
use session::{SessionStore, RedisStorage};


#[derive(Debug, Clone)]
pub struct WeChatClient {
    pub appid: String,
    pub secret: String,
    pub session: RedisStorage,
}

impl WeChatClient {

    #[inline]
    pub fn new(appid: &str, secret: &str, session: RedisStorage) -> WeChatClient {
        WeChatClient {
            appid: appid.to_owned(),
            secret: secret.to_owned(),
            session: session,
        }
    }

    #[inline]
    pub fn with_access_token(appid: &str, secret: &str, access_token: &str, session: RedisStorage) -> WeChatClient {
        let client = Self::new(appid, secret, session);
        let token_key = format!("{}_access_token", appid);
        client.session.set(&token_key, access_token.to_owned(), None);
        client
    }

    #[inline]
    pub fn access_token(&self) -> String {
        let token_key = format!("{}_access_token", self.appid);
        let token: String = self.session.get(&token_key, Some("".to_owned())).unwrap();
        token
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
        http_url.set_query_from_pairs(querys.into_iter());
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
            Err(_) => {
                error!("Send request error");
                return Err(WeChatError::ClientError { errcode: -1, errmsg: "Send request error".to_owned() });
            }
        };
        if res.status != hyper::Ok {
            error!("Response status error");
            return Err(WeChatError::ClientError { errcode: -2, errmsg: "Response status error".to_owned() })
        }
        Ok(res)
        
    }

    #[inline]
    fn json_decode(&self, res: &mut Response) -> WeChatResult<Json> {
        let obj = match Json::from_reader(res) {
            Ok(decoded) => { decoded },
            Err(_) => {
                error!("Json decode error");
                return Err(WeChatError::ClientError { errcode: -3, errmsg: "Json decode error".to_owned() });
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
        let mut res = try!(self.request(Method::Post, url, params, data));
        self.json_decode(&mut res)
    }

    #[inline]
    pub fn get(&self, url: &str, params: Vec<(&str, &str)>) -> WeChatResult<Json> {
        if self.access_token().is_empty() {
            self.fetch_access_token();
        }
        let mut res = try!(self.request(Method::Get, url, params, &Object::new()));
        self.json_decode(&mut res)
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
        let token_str = match *token {
            Json::String(ref v) => {
                let token_key = format!("{}_access_token", self.appid);
                self.session.set(&token_key, v.to_owned(), Some(7200usize));
                Some(format!("{}", v))
            },
            _ => None,
        };
        token_str
    }
}
