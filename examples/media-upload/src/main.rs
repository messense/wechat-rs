extern crate wechat;

use std::fs::File;

use wechat::WeChatClient;
use wechat::session::RedisStorage;


const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const REDIS_URI: &'static str = "redis://127.0.0.1/";


fn main() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);

    // let mut media_file = File::open("/Users/messense/Desktop/test.amr").unwrap();
    let mut media_file = File::open("/Users/messense/Pictures/1.png").unwrap();
    let res = client.media.upload("image", &mut media_file).unwrap();
    println!("{:?}", res);
}
