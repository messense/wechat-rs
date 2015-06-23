extern crate wechat;

use wechat::WeChatClient;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";

#[test]
fn test_fetch_access_token() {
    let client = WeChatClient::new(APPID, SECRET);
    let access_token = client.fetch_access_token();
    assert!(access_token.is_some());
    assert!(!client.access_token().is_empty());
}

#[test]
fn test_call_api_with_no_access_token_provided() {
    let client = WeChatClient::new(APPID, SECRET);
    let res = client.get("getcallbackip", vec![]);
    let data = match res {
        Ok(data) => data,
        Err(_) => { panic!("Error calling API"); },
    };
    let ip_list = data.find("ip_list").unwrap();
    let ips = ip_list.as_array().unwrap();
    assert!(ips.len() > 0);
}