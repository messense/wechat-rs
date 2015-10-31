use jsonway;

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

    let data = jsonway::object(|obj| {
        obj.array("button", |obj| {
            obj.push(jsonway::object(|btn| {
                btn.set("type", "click".to_owned());
                btn.set("key", "test".to_owned());
                btn.set("name", "test".to_owned());
            }).unwrap());
            obj.push(jsonway::object(|btn| {
                btn.set("type", "view".to_owned());
                btn.set("url", "http://www.qq.com".to_owned());
                btn.set("name", "QQ".to_owned());
            }).unwrap());
        });
    }).unwrap();
    println!("{:?}", data);
    // create new
    let res = client.menu.create(&data);
    println!("{:?}", res);
    assert!(res.is_ok());

    // try get
    let res = client.menu.get();
    assert!(res.is_ok());

    // try get current menu info
    let res = client.menu.get_menu_info();
    assert!(res.is_ok());

    // try update
    let res = client.menu.update(&data);
    assert!(res.is_ok());

    // cleanup
    let res = client.menu.delete();
    assert!(res.is_ok());
}
