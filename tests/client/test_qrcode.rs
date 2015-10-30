use wechat::WeChatClient;
use wechat::session::RedisStorage;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const REDIS_URI: &'static str = "redis://127.0.0.1/";

#[test]
fn test_qrcode_create_temp(){
    use wechat::client::request::TempQRCodeRequest;

    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);

    let req = TempQRCodeRequest::new(123, 600);
    let res = client.qrcode.create(&req);

    assert!(res.is_ok());

    let qrcode_url = client.qrcode.get_url(&res.unwrap());
    assert!(qrcode_url.len() > 0);
}

#[test]
fn test_qrcode_create_perm(){
    use wechat::client::request::PermQRCodeRequest;

    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);

    let req = PermQRCodeRequest::new("test");
    let res = client.qrcode.create(&req);

    assert!(res.is_ok());

    let qrcode_url = client.qrcode.get_url(&res.unwrap());
    assert!(qrcode_url.len() > 0);
}
