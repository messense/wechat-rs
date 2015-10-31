use jsonway;

use wechat::WeChatClient;
use wechat::session::RedisStorage;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const REDIS_URI: &'static str = "redis://127.0.0.1/";

#[test]
fn test_semantic_search() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);
    let query = jsonway::object(|obj| {
        obj.set("query", "故宫门票多少钱".to_owned());
        obj.set("category", "travel".to_owned());
        obj.set("city", "北京".to_owned());
        obj.set("appid", client.appid.to_owned());
    }).unwrap();
    let res = client.semantic.search(&query);
    assert!(res.is_ok());
}
