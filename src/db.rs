use redis;
use redis::RedisResult;

pub struct Db {
    con: redis::Connection,
    name: String
}
impl Db {
    pub fn new(name: &str) -> Db {
        let client = match redis::Client::open("redis://127.0.0.1") {
            Ok(client) => client,
            Err(err) => panic!("{}", err)
        };

        let con = match client.get_connection() {
            Ok(con) => con,
            Err(err) => panic!("{}", err)
        };

        Db{name:name.into(), con:con}
    }

    pub fn save(&self, id: &str, name: &str) -> RedisResult<String> {
        redis::cmd("HMSET").arg(self.key(id)).arg("name").arg(name).
            query(&self.con)
    }
    pub fn set(&self, key: &str, val: &str) -> RedisResult<String> {
        redis::cmd("SET").arg(key).arg(val).query(&self.con)
    }

    pub fn get(&self, id: &str) -> RedisResult<String> {
        redis::cmd("HGET").arg(self.key(id)).arg("name").query(&self.con)
    }

    pub fn get_ids(&self) -> RedisResult<Vec<String>> {
        let key = format!("{}:*", self.name);
        let res: Vec<String> = try!(redis::cmd("KEYS").arg(&key).query(&self.con));
        let mut ret: Vec<String> = Vec::new();
        for id in res {
            let (_, l) = id.split_at(&key.len() - 1);
            ret.push(l.into());
        }
        Ok(ret)
    }

    pub fn key(&self, id: &str) -> String {
        format!("{}:{}", &self.name, id)
    }
    pub fn key_name(&self, name: &str) -> String {
        format!("{}_name:{}", &self.name, name)
    }
}

