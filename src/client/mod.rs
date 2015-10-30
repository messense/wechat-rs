mod client;
pub mod api;
pub mod request;
pub mod response;

pub use self::client::APIClient;
use self::api::*;

use session::SessionStore;

#[derive(Debug, Clone)]
pub struct WeChatClient<T: SessionStore> {
    pub appid: String,
    pub secret: String,
    pub user: WeChatUser<T>,
    pub group: WeChatGroup<T>,
    pub menu: WeChatMenu<T>,
    pub message: WeChatMessage<T>,
    pub media: WeChatMedia<T>,
    pub material: WeChatMaterial<T>,
    pub misc: WeChatMisc<T>,
    pub qrcode: WeChatQRCode<T>,
    pub semantic: WeChatSemantic<T>,
    pub customservice: WeChatCustomService<T>,
}

impl<T: SessionStore> WeChatClient<T> {

    fn from_client(client: APIClient<T>) -> WeChatClient<T> {
        WeChatClient {
            appid: client.appid.clone(),
            secret: client.secret.clone(),
            user: WeChatUser::new(client.clone()),
            group: WeChatGroup::new(client.clone()),
            menu: WeChatMenu::new(client.clone()),
            message: WeChatMessage::new(client.clone()),
            media: WeChatMedia::new(client.clone()),
            material: WeChatMaterial::new(client.clone()),
            misc: WeChatMisc::new(client.clone()),
            qrcode: WeChatQRCode::new(client.clone()),
            semantic: WeChatSemantic::new(client.clone()),
            customservice: WeChatCustomService::new(client.clone()),
        }
    }

    pub fn new<S: Into<String>>(appid: S, secret: S, session: T) -> WeChatClient<T> {
        let client = APIClient::new(appid.into(), secret.into(), session);
        Self::from_client(client)
    }

    pub fn with_access_token<S: Into<String>>(appid: S, secret: S, access_token: S, session: T) -> WeChatClient<T> {
        let client = APIClient::with_access_token(appid.into(), secret.into(), access_token.into(), session);
        Self::from_client(client)
    }
}
