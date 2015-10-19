use rustc_serialize::Encodable;
use rustc_serialize::json::Json;

use session::SessionStore;
use client::{WeChatClient, WeChatResult};

#[derive(Debug, Clone)]
pub struct WeChatMenu<'a, T: SessionStore + 'a> {
    client: &'a WeChatClient<T>,
}

impl<'a, T: SessionStore> WeChatMenu<'a, T> {

    #[inline]
    pub fn new(client: &'a WeChatClient<T>) -> WeChatMenu<'a, T> {
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
