use wechat::WeChatClient;
use wechat::session::RedisStorage;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const REDIS_URI: &'static str = "redis://127.0.0.1/";

#[test]
fn test_misc_get_wechat_ips() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);
    let ips = client.misc.get_wechat_ips().unwrap();
    assert!(ips.len() > 0);
}

#[test]
fn test_misc_short_url() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);
    let url = client.misc.short_url("http://www.qq.com").unwrap();
    assert!(url.len() > 0);
}
