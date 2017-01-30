
#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate redis;

extern crate ms_speaker_recog;
extern crate rustc_serialize;

pub mod db;
pub mod ctrl;

fn main() {
    let api_key = include_str!("../api.txt"); 
    let client = ms_speaker_recog::Client::new(api_key, 0);
    let db = db::Db::new("speech_auth");
    let ctrl = ctrl::Ctrl::new(client, db);
    ctrl.identify();
}

