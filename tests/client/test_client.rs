use jsonway;

use wechat::WeChatClient;
use wechat::client::APIClient;
use wechat::session::RedisStorage;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const REDIS_URI: &'static str = "redis://127.0.0.1/";

#[test]
fn test_fetch_access_token() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = APIClient::new(APPID, SECRET, session);
    let access_token = client.fetch_access_token();
    assert!(access_token.is_some());
    assert!(!client.access_token().is_empty());
}

#[test]
fn test_call_api_with_no_access_token_provided() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = APIClient::new(APPID, SECRET, session);
    let res = client.get("getcallbackip", vec![]);
    let data = match res {
        Ok(data) => data,
        Err(_) => { panic!("Error calling API"); },
    };
    let ip_list = data.find("ip_list").unwrap();
    let ips = ip_list.as_array().unwrap();
    assert!(ips.len() > 0);
}

#[test]
fn test_call_api_with_access_token_provided() {
    let session0 = RedisStorage::from_url(REDIS_URI);
    let client0 = APIClient::new(APPID, SECRET, session0);
    let access_token = client0.fetch_access_token();
    assert!(access_token.is_some());

    let session1 = RedisStorage::from_url(REDIS_URI);
    let client = APIClient::with_access_token(APPID, SECRET, &access_token.unwrap(), session1);
    let res = client.get("getcallbackip", vec![]);
    let data = match res {
        Ok(data) => data,
        Err(_) => { panic!("Error calling API"); },
    };
    let ip_list = data.find("ip_list").unwrap();
    let ips = ip_list.as_array().unwrap();
    assert!(ips.len() > 0);
}

#[test]
fn test_call_get_api_with_invalid_access_token_auto_retry() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = APIClient::with_access_token(APPID, SECRET, "invalid access_token", session);
    let res = client.get("getcallbackip", vec![]);
    println!("{:?}", res);
    let data = match res {
        Ok(data) => data,
        Err(_) => { panic!("Error calling API"); },
    };
    let ip_list = data.find("ip_list").unwrap();
    let ips = ip_list.as_array().unwrap();
    assert!(ips.len() > 0);
}

#[test]
fn test_call_post_api_with_invalid_access_token_auto_retry() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::with_access_token(APPID, SECRET, "invalid access_token", session);
    let query = jsonway::object(|obj| {
        obj.set("query", "故宫门票多少钱".to_owned());
        obj.set("category", "travel".to_owned());
        obj.set("city", "北京".to_owned());
        obj.set("appid", client.appid.to_owned());
    }).unwrap();
    let res = client.semantic.search(&query);
    println!("{:?}", res);
    assert!(res.is_ok());
}
