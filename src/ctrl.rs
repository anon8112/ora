use db::Db;
use api::Api;
use rustc_serialize::json::Json;

use std::{time, thread};

pub struct Ctrl {
    db: Db,
    api: Api
}

impl Ctrl {
    pub fn new(api: Api, db: Db) -> Ctrl {
        Ctrl{api:api, db:db}
    }
    pub fn get_id(&self, id: &str) -> String {
        let res = &self.api.get_status(id).unwrap();
        let json = Json::from_str(&res).unwrap();
        let obj = json.as_object().unwrap();
        let r = obj.get("processingResult").unwrap();
        let id = r.as_object().unwrap().get("identifiedProfileId").unwrap();
        id.as_string().unwrap().into()
    }
    pub fn identify(&self) {
        let file = include_bytes!("../data/max/max_hello1.wav");
        let ids = &self.db.get_ids().unwrap();
        let res = self.api.identify(ids, file);
        let s = match res {
            Ok(res) => res,
            Err(err) => format!("{}",err)
        };
        let base = "https://westus.api.cognitive.microsoft.com/spid/v1.0/operations/";
        println!("{}", s);
        let (_, status_id) = s.split_at(base.len());
        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);
        let id = self.get_id(status_id);
        let name = match self.db.get(&id){
            Ok(name) => name,
            Err(err) => format!("{}", err)
        };
        println!("User {} is talking", name);
    }
    pub fn register(&self, name: &str) {
        let id = self.api.create_profile().unwrap();
        let _ = self.db.save(&id, name).unwrap();
        let _ = self.db.set(&self.db.key_name(name), &id).unwrap();
        
        let file = include_bytes!("../data/max/max1.wav");
        let res = self.api.enroll(&id, file).unwrap();
        println!("{}", res);

    }
}
