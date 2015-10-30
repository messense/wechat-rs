use rustc_serialize::Encodable;
use rustc_serialize::json::Json;

use types::WeChatResult;
use client::APIClient;
use session::SessionStore;


#[derive(Debug, Clone)]
pub struct WeChatMenu<T: SessionStore> {
    client: APIClient<T>,
}

impl<T: SessionStore> WeChatMenu<T> {

    #[inline]
    pub fn new(client: APIClient<T>) -> WeChatMenu<T> {
        WeChatMenu {
            client: client,
        }
    }

    pub fn create<D: Encodable>(&self, data: &D) -> WeChatResult<()> {
        try!(self.client.post("menu/create", vec![], data));
        Ok(())
    }

    pub fn get(&self) -> WeChatResult<Json> {
        self.client.get("menu/get", vec![])
    }

    pub fn delete(&self) -> WeChatResult<()> {
        try!(self.client.get("menu/delete", vec![]));
        Ok(())
    }

    pub fn update<D: Encodable>(&self, data: &D) -> WeChatResult<()> {
        try!(self.delete());
        self.create(data)
    }

    pub fn get_menu_info(&self) -> WeChatResult<Json> {
        self.client.get("get_current_selfmenu_info", vec![])
    }
}
