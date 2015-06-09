#![feature(collections)]
extern crate time;
extern crate rand;
extern crate crypto;
extern crate sxd_document;
extern crate sxd_xpath;
extern crate rustc_serialize;
extern crate byteorder;

mod xmlutil;
mod cryptography;
pub mod messages;
mod events;
mod parser;
mod errors;

pub use self::messages::MessageParser;
pub use self::parser::parse_message;
pub use self::errors::WeChatError;
pub use self::cryptography::WeChatCrypto;
