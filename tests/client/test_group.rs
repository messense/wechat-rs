use wechat::WeChatClient;
use wechat::session::RedisStorage;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";
const REDIS_URI: &'static str = "redis://127.0.0.1/";

#[test]
fn test_group_create_update_and_delete() {
    let session = RedisStorage::from_url(REDIS_URI);
    let client = WeChatClient::new(APPID, SECRET, session);

    // create group
    let res = client.group.create("测试分组");
    assert!(res.is_ok());

    let group = res.unwrap();
    let group_id = group.id;

    // update group name
    let res = client.group.update(group_id, "Test Group");
    assert!(res.is_ok());

    let res = client.group.list();
    assert!(res.is_ok());
    let groups = res.unwrap();
    assert!(groups.len() > 0);

    // delete group
    let _ = client.group.delete(group_id);
}
