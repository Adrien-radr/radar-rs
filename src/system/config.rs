use system::rustc_serialize::json;
use system::filesystem;
use std::path::Path;

pub struct Config {
    json_conf: json::Json
}

impl Config {
    pub fn new(path: &str) -> Config {
        let conf_str = filesystem::read_file(path);

        // decode
        let json = match json::Json::from_str(&conf_str) {
            Ok(o) => o,
            Err(msg) => panic!("Error parsing Json config file: {}", msg)
        };

        Config {
            json_conf: json
        }
    }

    fn get_obj<T>(&self, field: &str) -> &json::Json {
        match self.json_conf.as_object().unwrap().get(field) {
            Some(f) => f,
            None => panic!("Couldn't retrieve field {} from config file.", field)
        }
    }

    pub fn get_f64(&self, field: &str) -> f64 {
        self.get_obj::<f64>(field).as_f64().unwrap()
    }

    pub fn get_u64(&self, field: &str) -> u64 {
        self.get_obj::<u64>(field).as_u64().unwrap()
    }

    pub fn get_i64(&self, field: &str) -> i64 {
        self.get_obj::<i64>(field).as_i64().unwrap()
    }
}