use jsonway;

use types::WeChatResult;
use client::APIClient;
use client::response::Group;
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatGroup<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatGroup<T> {

    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatGroup<T> {
        WeChatGroup {
            client: client,
        }
    }

    pub fn create(&self, name: &str) -> WeChatResult<Group> {
        let data = jsonway::object(|obj| {
            obj.object("group", |obj| {
                obj.set("name", name.to_owned());
            });
        }).unwrap();
        let res = try!(self.client.post("groups/create", vec![], &data));
        let group_id = res.find_path(&["group", "id"]).unwrap();
        let group_id = group_id.as_u64().unwrap();
        let group_name = res.find_path(&["group", "name"]).unwrap();
        let group_name = group_name.as_string().unwrap();
        Ok(Group {
            id: group_id,
            name: group_name.to_owned(),
            count: 0u64,
        })
    }

    pub fn list(&self) -> WeChatResult<Vec<Group>> {
        let res = try!(self.client.get("groups/get", vec![]));
        let groups = res.find("groups").unwrap();
        let groups_array = groups.as_array().unwrap();
        let mut groups = vec![];
        for group_json in groups_array {
            let group_id = &group_json["id"];
            let group_id = group_id.as_u64().unwrap();
            let group_name = &group_json["name"];
            let group_name = group_name.as_string().unwrap();
            let group_count = &group_json["count"];
            let group_count = group_count.as_u64().unwrap();
            let group = Group {
                id: group_id,
                name: group_name.to_owned(),
                count: group_count,
            };
            groups.push(group);
        }
        Ok(groups)
    }

    pub fn update(&self, group_id: u64, name: &str) -> WeChatResult<()> {
        let data = jsonway::object(|obj| {
            obj.object("group", |obj| {
                obj.set("id", group_id);
                obj.set("name", name.to_owned());
            });
        }).unwrap();
        try!(self.client.post("groups/update", vec![], &data));
        Ok(())
    }

    pub fn delete(&self, group_id: u64) -> WeChatResult<()> {
        let data = jsonway::object(|obj| {
            obj.object("group", |obj| {
                obj.set("id", group_id);
            });
        }).unwrap();
        try!(self.client.post("groups/delete", vec![], &data));
        Ok(())
    }

    pub fn move_user(&self, openid: &str, group_id: u64) -> WeChatResult<()> {
        let data = jsonway::object(|obj| {
            obj.set("openid", openid.to_owned());
            obj.set("to_groupid", group_id);
        }).unwrap();
        try!(self.client.post("groups/members/update", vec![], &data));
        Ok(())
    }

    pub fn move_users(&self, openids: Vec<String>, group_id: u64) -> WeChatResult<()> {
        let data = jsonway::object(|obj| {
            obj.set("openid_list", openids);
            obj.set("to_groupid", group_id);
        }).unwrap();
        try!(self.client.post("groups/members/batchupdate", vec![], &data));
        Ok(())
    }
}
