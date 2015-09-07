#![allow(dead_code)]
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use super::base_task::BaseTask;

pub struct Error {
    code: Option<i32>,
    message: Option<String>,
}

impl ToJson for Error {
    fn to_json(&self) -> Json {
        let mut map: BTreeMap<String, Json> = BTreeMap::new();
        map.insert("code".to_string(), self.code.to_json());
        map.insert("message".to_string(), self.message.to_json());
        map.to_json()
    }
}

impl Error {
    pub fn new() -> Error {
        Error {
            code: None,
            message: None,
        }
    }
    pub fn set_code(&mut self, code: i32) {
        self.code = Some(code);
    }
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }
}
