#![allow(dead_code)]
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use super::text::TextTask;
use super::image::ImageTask;
use super::location::LocationTask;

#[derive(Debug, Clone)]
pub enum YJComponentType {
    YJRadio(YJRadio),
}

#[derive(Debug, Clone)]
pub struct YJRadio {
    label: Option<String>,
    value: Option<String>,
}

pub struct WebpageTask {
    url: Option<String>,
    type_name: Option<String>,
    title: Option<String>,
    description: Option<String>,
    data_type: Option<String>,
    data: Option<Vec<YJComponentType>>
}
impl ToJson for YJComponentType {
    fn to_json (&self) -> Json {
        let yj_component = self.clone();
        match yj_component {
            YJComponentType::YJRadio(yj_radio) => {
                let mut map: BTreeMap<String, Json> = BTreeMap::new();
                map.insert("label".to_string(), yj_radio.label.to_json());
                map.insert("value".to_string(), yj_radio.value.to_json());
                map.to_json()
            }
        }

    }
}
impl ToJson for WebpageTask {
    fn to_json(&self) -> Json {
        let mut map: BTreeMap<String, Json> = BTreeMap::new();
        map.insert("url".to_string(), self.url.to_json());
        let mut config: BTreeMap<String, Json> = BTreeMap::new();
        if self.type_name != None {
            config.insert("type".to_string(), self.type_name.to_json());
            config.insert("title".to_string(), self.title.to_json());
            config.insert("description".to_string(), self.description.to_json());
            config.insert("dataType".to_string(), self.data_type.to_json());
            config.insert("data".to_string(), self.data.to_json());
            map.insert("config".to_string(), config.to_json());
        }
        map.to_json()
    }
}

impl WebpageTask {
    pub fn new() -> WebpageTask {
        WebpageTask{
            url: None,
            type_name: None,
            title: None,
            description: None,
            data_type: None,
            data: None,
        }
    }


    pub fn set_url(& mut self, url: String) {
            self.url = Some(url);
    }
    pub fn set_type_name(& mut self, type_name: String) {
        self.type_name = Some(type_name);
    }
    pub fn set_title(& mut self, title: String) {
        self.title = Some(title);
    }
    pub fn set_description(& mut self, description: String) {
        self.description = Some(description);
    }
    pub fn set_data_type(& mut self, data_type: String) {
        self.data_type = Some(data_type);
    }
    pub fn set_data(& mut self, data: Vec<YJComponentType>) {
        self.data = Some(data);
    }

}
