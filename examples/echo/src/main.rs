#[macro_use]
extern crate log;
extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate bodyparser;
extern crate env_logger;
extern crate wechat;

use std::collections::HashMap;

use iron::prelude::*;
use iron::status;
use iron::method::Method;
use router::Router;
use urlencoded::UrlEncodedQuery;

use wechat::{check_signature, Message};
use wechat::replies::{TextReply, ReplyRenderer};


const TOKEN: &'static str = "123456";


fn get_query_string<S: Into<String>>(map: &HashMap<String, Vec<String>>, key: &str, default: S) -> String {
    let query = match map.get(key) {
        Some(vec) => {
            match vec.first() {
                Some(val) => val.to_owned(),
                None => default.into(),
            }
        },
        None => default.into(),
    };
    query
}


fn wechat_callback_handler(req: &mut Request) -> IronResult<Response> {
    let qs = match req.get::<UrlEncodedQuery>() {
        Ok(hashmap) => hashmap,
        Err(_) => HashMap::new(),
    };
    let signature = get_query_string(&qs, "signature", "");
    let timestamp = get_query_string(&qs, "timestamp", "");
    let nonce = get_query_string(&qs, "nonce", "");

    if !check_signature(TOKEN, &signature, &timestamp, &nonce) {
        println!("Check signature failed!");
        return Ok(Response::with(status::Unauthorized));
    }

    match req.method {
        Method::Get => {
            info!("Check signature succeed!");
            let echo_str = get_query_string(&qs, "echostr", "");
            return Ok(Response::with((status::Ok, echo_str)));
        },
        Method::Post => {
            let body = match req.get::<bodyparser::Raw>() {
                Ok(Some(body)) => body,
                Ok(None) => "".to_owned(),
                Err(_) => "".to_owned(),
            };
            let msg = Message::parse(&body);
            info!("New message: {:?}", msg);

            match msg {
                Message::TextMessage(msg) => {
                    let reply = TextReply::new(msg.target, msg.source, msg.content);
                    return Ok(Response::with((status::Ok, reply.render())));
                },
                _ => {
                    let reply = TextReply::new(msg.get_target(), msg.get_source(), "Sorry, can not handle this for now.".to_owned());
                    return Ok(Response::with((status::Ok, reply.render())));
                },
            }
        },
        _ => {},
    }
    Ok(Response::with(status::Ok))
}


fn main() {
    // Enable logging
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/callback", wechat_callback_handler);
    router.post("/callback", wechat_callback_handler);

    info!("Listening on localhost:5000");
    Iron::new(router).http("localhost:5000").unwrap();
}
