use wechat::WeChatClient;
use wechat::client::WeChatGroup;

const APPID: &'static str = "wxd7aa56e2c7b1f4f1";
const SECRET: &'static str = "2817b66a1d5829847196cf2f96ab2816";

#[test]
fn test_group_create_update_and_delete() {
    let client = WeChatClient::new(APPID, SECRET);
    let group = WeChatGroup::new(&client);

    // create group
    let res = group.create("测试分组");
    assert!(res.is_ok());

    let res = res.unwrap();
    let group_id = res.find_path(&["group", "id"]).unwrap();
    let group_id = group_id.as_u64().unwrap();

    // update group name
    let res = group.update(group_id, "Test Group");
    assert!(res.is_ok());

    // delete group
    let _ = group.delete(group_id);
}