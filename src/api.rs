
use hyper;
use hyper::client;
use hyper::header::Headers;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

header! { (ContentType, "Content-Type") => [String] }
header! { (SubscriptionKey, "Ocp-Apim-Subscription-Key") => [String] }
header! { (OperationLocation, "Operation-Location") => [String] }
use rustc_serialize::json::Json;


pub struct Api {
    base_url: String,
    key: String,
    client: client::Client
}

impl Api {
    pub fn init(key: &str) -> Api {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        
       
        Api{base_url: "https://westus.api.cognitive.microsoft.com/spid/v1.0/".into(),
            key: key.into(),
            client: client::Client::with_connector(connector)}
    }

    pub fn get_status(&self, id: &str) -> Result<String, hyper::Error> {
        let params = format!("operations/{}", id);
        let res = try!(self.get_request(&params));
        Ok(res)
    }

    pub fn create_profile(&self) -> Result<String, hyper::Error> {
        let res = try!(self.request("identificationProfiles","{\"locale\":\"en-us\"}"));
        let json = Json::from_str(&res).unwrap();
        let obj = json.as_object().unwrap();
        let id = obj.get("identificationProfileId").unwrap();
        Ok(id.as_string().unwrap().into())
    }

    pub fn enroll(&self, id: &str, audio: &[u8]) -> Result<String, hyper::Error> {
        let params = format!("identificationProfiles/{}/enroll", id);
        let res = try!(self.audio_request(&params, audio));
        Ok(format!("{}", res)) 
    }

    pub fn identify(&self, ids: &[String], audio: &[u8]) -> Result<String, hyper::Error> {
        let mut param = String::new();
        for id in ids {
            param = format!("{},{}", param, id);
        }
        let (_, l) = param.split_at(1);
        println!("{}", l);
        let params = format!("identify?identificationProfileIds={}&shortAudio=true",
                             l);
        let res = try!(self.audio_request(&params, audio));
        Ok(res)

    }

    fn get_request(&self, op: &str) -> Result<String, hyper::Error> {
        let mut headers = Headers::new();
        headers.set(ContentType("application/json".into()));
        headers.set(SubscriptionKey(self.key.clone()));

        let res = self.client.get(&format!("{}{}",&self.base_url, op))
            .headers(headers).send();
        let mut s = String::new();
        
        match res {
            Ok(mut res) => {res.read_to_string(&mut s).unwrap(); ()},
            Err(err) => return Err(err)
        };
        Ok(s)
    }

    fn request(&self, op: &str, data: &str) -> Result<String, hyper::Error> {
        let mut headers = Headers::new();
        headers.set(ContentType("application/json".into()));
        headers.set(SubscriptionKey(self.key.clone()));

        let res = self.client.post(&format!("{}{}",&self.base_url, op))
            .body(data).headers(headers).send();
        let mut s = String::new();
        
        match res {
            Ok(mut res) => {res.read_to_string(&mut s).unwrap(); ()},
            Err(err) => return Err(err)
        };
        Ok(s)
    }
    
    fn audio_request(&self, op: &str, data: &[u8]) -> Result<String, hyper::Error> {
        let mut headers = Headers::new();
        headers.set(ContentType("multipart/form-data".into()));
        headers.set(SubscriptionKey(self.key.clone()));

        let res = self.client.post(&format!("{}{}",&self.base_url, op))
            .body(data).headers(headers).send();
        let mut s = String::new();
        
        match res {
            Ok(mut res) => {
                res.read_to_string(&mut s).unwrap();
                
                println!("{}", s);
                s = format!("{}", String::from_utf8_lossy(
                             &res.headers.get_raw("Operation-Location").unwrap()[0]));
                ()
            },
            Err(err) => return Err(err)
        };
        Ok(s)
    }

}
