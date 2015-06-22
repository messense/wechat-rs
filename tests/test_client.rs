extern crate wechat;

use wechat::WeChatClient;

#[test]
fn test_fetch_access_token() {
    let appid = "wxd7aa56e2c7b1f4f1";
    let secret = "2817b66a1d5829847196cf2f96ab2816";
    let mut client = WeChatClient::new(appid, secret);
    let access_token = client.fetch_access_token();
    assert!(access_token.is_some());
    assert!(!client.access_token.is_empty());
}