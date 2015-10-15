use rustc_serialize::json::{Json, ToJson};


#[derive(Debug, Clone)]
pub struct TempQRCodeRequest {
    scene_id: u32,
    expire_seconds: u32,
}

#[derive(Debug, Clone)]
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
        json!({
            "action_name": "QR_SCENE",
            "expire_seconds": (self.expire_seconds),
            "action_info": {
                "scene": {"scene_id": (self.scene_id)}
            }
        })
    }
}

impl PermQRCodeRequest {
    pub fn new(scene_str: &str) -> PermQRCodeRequest {
        PermQRCodeRequest  {
            scene_str: scene_str.to_owned(),
        }
    }
}

impl ToJson for PermQRCodeRequest {
    fn  to_json(&self) -> Json {
        json!({
            "action_name": "QR_LIMIT_STR_SCENE",
            "action_info": {
                "scene": {"scene_str": (self.scene_str)}
            }
        })
    }
}
