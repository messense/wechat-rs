use wechat::WeChatClient;
use wechat::client::WeChatSemantic;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";

#[test]
fn test_semantic_search() {
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