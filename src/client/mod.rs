use url::Url;
use hyper::{self, Client};
use hyper::method::Method;
use rustc_serialize::json::{self, Json, Object};

use errors::WeChatError;

//mod misc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WeChatClient {
    pub appid: String,
    pub secret: String,
    pub access_token: String,
}

impl WeChatClient {

    #[inline]
    pub fn new(appid: &str, secret: &str) -> WeChatClient {
        WeChatClient {
            appid: appid.to_owned(),
            secret: secret.to_owned(),
            access_token: "".to_owned(),
        }
    }

    #[inline]
    pub fn with_access_token(appid: &str, secret: &str, access_token: &str) -> WeChatClient {
        WeChatClient {
            appid: appid.to_owned(),
            secret: secret.to_owned(),
            access_token: access_token.to_owned(),
        }
    }

    pub fn request(&self, method: Method, url: &str, params: Vec<(&str, &str)>, data: &Object) -> Result<Json, WeChatError> {
        let mut http_url = if !(url.starts_with("http://") || url.starts_with("https://")) {
            let mut url_string = "https://api.weixin.qq.com/cgi-bin/".to_owned();
            url_string = url_string + url;
            Url::parse(&url_string).unwrap()
        } else {
            Url::parse(url).unwrap()
        };
        let mut querys = params.clone();
        if !self.access_token.is_empty() {
            debug!("Using access_token: {}", self.access_token);
            querys.push(("access_token", &self.access_token));
        }
        http_url.set_query_from_pairs(querys.into_iter());
        let body = if !data.is_empty() {
            match json::encode(data) {
                Ok(text) => text,
                Err(_) => "".to_owned(),
            }
        } else {
            "".to_owned()
        };
        let mut client = Client::new();
        let req = if method == Method::Post {
            client.post(http_url).body(&body)
        } else {
            client.get(http_url)
        };
        let mut res = match req.send() {
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
        let obj = match Json::from_reader(&mut res) {
            Ok(decoded) => { decoded },
            Err(_) => {
                error!("Json decode error");
                return Err(WeChatError::ClientError { errcode: -3, errmsg: "Json decode error".to_owned() });
            }
        };
        Ok(obj)
    }

    #[inline]
    pub fn post(&mut self, url: &str, params: Vec<(&str, &str)>, data: &Object) -> Result<Json, WeChatError> {
        if self.access_token.is_empty() {
            self.fetch_access_token();
        }
        self.request(Method::Post, url, params, data)
    }

    #[inline]
    pub fn get(&mut self, url: &str, params: Vec<(&str, &str)>) -> Result<Json, WeChatError> {
        if self.access_token.is_empty() {
            self.fetch_access_token();
        }
        self.request(Method::Get, url, params, &Object::new())
    }

    pub fn fetch_access_token(&mut self) -> Option<String> {
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
        let data = match res {
            Ok(data) => data,
            Err(_) => { return None; },
        };
        let token = match data.find("access_token") {
            Some(token) => token,
            None => { return None; }
        };
        let token_str = match *token {
            Json::String(ref v) => {
                self.access_token = v.to_owned();
                Some(format!("{}", v))
            },
            _ => None,
        };
        token_str
    }
}
