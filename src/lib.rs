#![feature(plugin)]
#![plugin(json_macros)]

extern crate time;
extern crate rand;
extern crate url;
extern crate sxd_document;
extern crate sxd_xpath;
extern crate rustc_serialize;
extern crate byteorder;
extern crate hyper;
extern crate openssl;
extern crate redis;
#[macro_use]
extern crate log;

#[macro_use]
mod macros;

mod xmlutil;
mod cryptography;
mod events;
mod parser;
mod errors;
mod utils;

pub mod messages;
pub mod replies;
pub mod client;
pub mod session;

pub use self::messages::MessageParser;
pub use self::messages::Message;
pub use self::parser::parse_message;
pub use self::errors::WeChatError;
pub use self::cryptography::WeChatCrypto;
pub use self::replies::ReplyRenderer;
pub use self::replies::Reply;
pub use self::client::WeChatClient;
pub use self::utils::check_signature;
