use errors::WeChatError;

pub type WeChatResult<T> = Result<T, WeChatError>;
