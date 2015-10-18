use wechat::WeChatClient;
use wechat::client::WeChatSemantic;
use wechat::session::RedisStorage;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const REDIS_URI: &'static str = "redis://127.0.0.1/";

#[test]
fn test_semantic_search() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);
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
