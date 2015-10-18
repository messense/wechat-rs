use errors::WeChatError;

pub type WeChatResult<T> = Result<T, WeChatError>;

mod client;
pub mod api;
pub mod request;
pub mod response;

pub use self::client::WeChatClient;
pub use self::api::*;
