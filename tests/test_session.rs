extern crate wechat;
extern crate redis;

use wechat::session::{SessionStore, RedisStorage};

const REDIS_URI: &'static str = "redis://127.0.0.1/";


#[test]
fn test_redis_storage_set_then_get_del() {
    let client = redis::Client::open(REDIS_URI).unwrap();
    let session = RedisStorage::new(client);

    session.del("test_session_1");
    session.set("test_session_1", 1usize, None);
    let data: usize = session.get("test_session_1", None).unwrap();
    assert_eq!(1usize, data);
    session.del("test_session_1");
}

#[test]
fn test_redis_storage_get_without_default() {
    let client = redis::Client::open(REDIS_URI).unwrap();
    let session = RedisStorage::new(client);

    session.del("test_session_2");
    let data: Option<usize> = session.get("test_session_2", None);
    assert!(data.is_none());
}

#[test]
fn test_redis_storage_get_with_default() {
    let client = redis::Client::open(REDIS_URI).unwrap();
    let session = RedisStorage::new(client);

    session.del("test_session_3");
    let data: Option<usize> = session.get("test_session_3", Some(1usize));
    let data = data.unwrap();
    assert_eq!(1usize, data);
}
