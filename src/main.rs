
#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate redis;

extern crate rustc_serialize;

pub mod db;
pub mod api;
pub mod ctrl;

fn main() {
    let api_key = include_str!("../api.txt"); 
    let api = api::Api::init(api_key);
    let db = db::Db::new("speech_auth");
    let ctrl = ctrl::Ctrl::new(api, db);
    ctrl.identify();
}

