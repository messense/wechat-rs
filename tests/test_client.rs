#![feature(plugin)]
#![plugin(json_macros)]
extern crate rustc_serialize;
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

#[test]
fn test_call_api_with_access_token_provided() {
    let client0 = WeChatClient::new(APPID, SECRET);
    let access_token = client0.fetch_access_token();

    let client = WeChatClient::with_access_token(APPID, SECRET, &access_token.unwrap());
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
fn test_misc_get_wechat_ips() {
    use wechat::client::WeChatMisc;

    let client = WeChatClient::new(APPID, SECRET);
    let misc = WeChatMisc::new(&client);
    let ips = misc.get_wechat_ips().unwrap();
    assert!(ips.len() > 0);
}

#[test]
fn test_misc_short_url() {
    use wechat::client::WeChatMisc;

    let client = WeChatClient::new(APPID, SECRET);
    let misc = WeChatMisc::new(&client);
    let url = misc.short_url("http://www.qq.com").unwrap();
    assert!(url.len() > 0);
}

#[test]
fn test_semantic_search_simple() {
    use wechat::client::WeChatSemantic;

    let client = WeChatClient::new(APPID, SECRET);
    let semantic = WeChatSemantic::new(&client);
    let res = semantic.search_simple("查一下明天从北京到上海的南航机票", "flight,hotel");
    assert!(res.is_ok());
}

#[test]
fn test_qrcode_create() {
    use wechat::client::WeChatQRCode;

    let client = WeChatClient::new(APPID, SECRET);
    let qrcode = WeChatQRCode::new(&client);
    let data = json!({
        "action_name": "QR_SCENE",
        "expire_seconds": 600,
        "action_info": {
            "scene": {
                "scene_id": 123
            }
        }
    });
    let res = qrcode.create(data.as_object().unwrap());
    assert!(res.is_ok());
}