#![feature(plugin)]
#![plugin(json_macros)]
extern crate rustc_serialize;
extern crate wechat;

use wechat::WeChatClient;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const OPENID: &'static str = "ozJS1syaqn5ztglMsr8ceH8o2zCQ";

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
    assert!(access_token.is_some());

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
fn test_semantic_search() {
    use wechat::client::WeChatSemantic;

    let client = WeChatClient::new(APPID, SECRET);
    let semantic = WeChatSemantic::new(&client);
    let query = json!({
        "query": "故宫门票多少钱",
        "category": "travel",
        "city": "北京",
        "appid": (client.appid)
    });
    let res = semantic.search(&query);
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
    let res = qrcode.create(&data);
    assert!(res.is_ok());

    let qrcode_url = WeChatQRCode::get_url(&res.unwrap());
    assert!(qrcode_url.len() > 0);
}

#[test]
fn test_menu_apis() {
    use wechat::client::WeChatMenu;

    let client = WeChatClient::new(APPID, SECRET);
    let menu = WeChatMenu::new(&client);

    // delete first
    let res = menu.delete();
    assert!(res.is_ok());

    // create new
    let res = menu.create(&json!({
        "button": [
            {"type": "click", "key": "test", "name": "test"},
            {"type": "view", "url": "http://www.qq.com", "name": "QQ"}
        ]
    }));
    assert!(res.is_ok());

    // try get
    let res = menu.get();
    assert!(res.is_ok());

    // try get current menu info
    let res = menu.get_menu_info();
    assert!(res.is_ok());

    // try update
    let res = menu.update(&json!({
        "button": [
            {"type": "click", "key": "test", "name": "test"},
            {"type": "view", "url": "http://www.qq.com", "name": "QQ"}
        ]
    }));
    assert!(res.is_ok());

    // cleanup
    let res = menu.delete();
    assert!(res.is_ok());
}

#[test]
fn test_user_get() {
    use wechat::client::WeChatUser;

    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get(OPENID);
    assert!(res.is_ok());
}

#[test]
fn test_user_get_with_lang() {
    use wechat::client::WeChatUser;

    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get_with_lang(OPENID, "zh_CN");
    assert!(res.is_ok());
}

#[test]
fn test_user_update_remark() {
    use wechat::client::WeChatUser;

    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.update_remark(OPENID, "test user");
    assert!(res.is_ok());
}

#[test]
fn test_user_get_followers_with_no_next_openid() {
    use wechat::client::WeChatUser;

    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get_followers("");
    assert!(res.is_ok());
}

#[test]
fn test_user_get_followers_with_next_openid() {
    use wechat::client::WeChatUser;

    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get_followers(OPENID);
    assert!(res.is_ok());
}

#[test]
fn test_user_get_group_id() {
    use wechat::client::WeChatUser;

    let client = WeChatClient::new(APPID, SECRET);
    let user = WeChatUser::new(&client);

    let res = user.get_group_id(OPENID);
    assert!(res.is_ok());
}

#[test]
fn test_group_create_update_and_delete() {
    use wechat::client::WeChatGroup;

    let client = WeChatClient::new(APPID, SECRET);
    let group = WeChatGroup::new(&client);

    // create group
    let res = group.create("测试分组");
    assert!(res.is_ok());

    let res = res.unwrap();
    let group_id = res.find_path(&["group", "id"]).unwrap();
    let group_id = group_id.as_u64().unwrap();

    // update group name
    let res = group.update(group_id, "Test Group");
    assert!(res.is_ok());

    // delete group
    let _ = group.delete(group_id);
}
