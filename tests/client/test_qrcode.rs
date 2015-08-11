use wechat::WeChatClient;
use wechat::client::WeChatQRCode;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";

#[test]
fn test_qrcode_create() {
    let client = WeChatClient::new(APPID, SECRET);
    let qrcode = WeChatQRCode::new(&client);
    let data = json!({
        "action_name": "QR_SCENE",
        "expire_seconds": 600,
        "action_info": {
            "scene": {
                "scene_id": 123
            }
        }
    });
    let res = qrcode.create(&data);
    assert!(res.is_ok());

    let qrcode_url = WeChatQRCode::get_url(&res.unwrap());
    assert!(qrcode_url.len() > 0);
}