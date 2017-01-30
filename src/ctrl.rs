use db::Db;

use std::{time, thread};
use ms_speaker_recog::Client;

pub struct Ctrl {
    api: Client,
    db: Db
}

impl Ctrl {
    pub fn new(api: Client, db: Db) -> Ctrl {
        Ctrl{api:api, db:db}
    }
    
    pub fn identify(&self) {
        let file = include_bytes!("../data/max/max_hello1.wav");
        let ids = &self.db.get_ids().unwrap();
        let res = self.api.identify(ids, file, true);
        let id = match res {
            Ok(res) => res,
            Err(err) => format!("{}", err.message)
        };
        thread::sleep(time::Duration::from_secs(2));
        let id = self.api.get_operation_status_id(&id).unwrap();
        let name = match self.db.get(&id){
            Ok(name) => name,
            Err(err) => format!("{}", err)
        };
        println!("User {} is talking", name);
    }
    pub fn register(&self, name: &str) {
        let id = self.api.create_profile("en-us").unwrap();
        let _ = self.db.save(&id, name).unwrap();
        let _ = self.db.set(&self.db.key_name(name), &id).unwrap();
        
        let file = include_bytes!("../data/max/max1.wav");
        let res = self.api.create_enrollment(&id, file, false).unwrap();
        println!("{}", res);

    }
}
