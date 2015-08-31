#![allow(dead_code)]
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use super::base_task::BaseTask;

pub struct TextTask{
    text: Option<String>,
}
impl ToJson for TextTask {
    fn to_json(&self) -> Json {
        self.text.to_json()
    }
}
impl TextTask {
    pub fn new() -> TextTask {
        TextTask{
            text: None,
        }
    }
    pub fn set_text(&mut self, text: String) {
        self.text = Some(text);
    }
}




// pub fn get_express_route(num: &str) -> Option<Json>{
//     let origin_url: String  = "http://m.kuaidi100.com/result.jsp?nu=".to_string();
//
//     Some(origin_url + num)
// }
