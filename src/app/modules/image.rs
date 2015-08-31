#![allow(dead_code)]
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

pub struct ImageTask {
    base64: Option<String>,
}
impl ToJson for ImageTask {
    fn to_json(&self) -> Json{
        self.base64.to_json()
    }
}
impl ImageTask {
    pub fn new () -> ImageTask {
        ImageTask{
            base64: None,
        }
    }
    pub fn set_base64(&mut self, base64: String) {
        self.base64 = Some(base64);
    }
}
