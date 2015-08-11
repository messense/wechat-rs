use wechat::WeChatClient;
use wechat::client::WeChatMisc;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";

#[test]
fn test_misc_get_wechat_ips() {
    let client = WeChatClient::new(APPID, SECRET);
    let misc = WeChatMisc::new(&client);
    let ips = misc.get_wechat_ips().unwrap();
    assert!(ips.len() > 0);
}

#[test]
fn test_misc_short_url() {
    let client = WeChatClient::new(APPID, SECRET);
    let misc = WeChatMisc::new(&client);
    let url = misc.short_url("http://www.qq.com").unwrap();
    assert!(url.len() > 0);
}