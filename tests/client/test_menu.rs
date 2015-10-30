use wechat::WeChatClient;
use wechat::session::RedisStorage;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const REDIS_URI: &'static str = "redis://127.0.0.1/";

#[test]
fn test_menu_apis() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);

    // delete first
    let res = client.menu.delete();
    assert!(res.is_ok());

    // create new
    let res = client.menu.create(&json!({
        "button": [
            {"type": "click", "key": "test", "name": "test"},
            {"type": "view", "url": "http://www.qq.com", "name": "QQ"}
        ]
    }));
    assert!(res.is_ok());

    // try get
    let res = client.menu.get();
    assert!(res.is_ok());

    // try get current menu info
    let res = client.menu.get_menu_info();
    assert!(res.is_ok());

    // try update
    let res = client.menu.update(&json!({
        "button": [
            {"type": "click", "key": "test", "name": "test"},
            {"type": "view", "url": "http://www.qq.com", "name": "QQ"}
        ]
    }));
    assert!(res.is_ok());

    // cleanup
    let res = client.menu.delete();
    assert!(res.is_ok());
}
