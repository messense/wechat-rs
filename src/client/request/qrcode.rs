use rustc_serialize::json::{Json, ToJson};
use jsonway;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TempQRCodeRequest {
    scene_id: u32,
    expire_seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermQRCodeRequest {
    scene_str: String,
}

impl TempQRCodeRequest {
    pub fn new(scene_id: u32, expire_seconds: u32) -> TempQRCodeRequest {
        TempQRCodeRequest {
            scene_id: scene_id,
            expire_seconds: expire_seconds,
        }
    }
}

impl ToJson for TempQRCodeRequest {
    fn to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("action_name", "QR_SCENE".to_owned());
            obj.set("expire_seconds", self.expire_seconds);
            obj.object("action_info", |obj| {
                obj.object("scene", |obj| {
                    obj.set("scene_id", self.scene_id);
                });
            });
        }).unwrap()
    }
}

make_encodable!(TempQRCodeRequest);

impl PermQRCodeRequest {
    pub fn new<S: Into<String>>(scene_str: S) -> PermQRCodeRequest {
        PermQRCodeRequest  {
            scene_str: scene_str.into(),
        }
    }
}

impl ToJson for PermQRCodeRequest {
    fn  to_json(&self) -> Json {
        jsonway::object(|obj| {
            obj.set("action_name", "QR_LIMIT_STR_SCENE".to_owned());
            obj.object("action_info", |obj| {
                obj.object("scene", |obj| {
                    obj.set("scene_str", self.scene_str.to_owned());
                });
            });
        }).unwrap()
    }
}

make_encodable!(PermQRCodeRequest);
