#![feature(plugin)]
#![plugin(json_macros)]
extern crate wechat;
extern crate redis;
extern crate rustc_serialize;

mod test_parser;
mod client;
mod test_session;
