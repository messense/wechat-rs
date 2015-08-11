use wechat::WeChatClient;
use wechat::client::WeChatMenu;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";

#[test]
fn test_menu_apis() {
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