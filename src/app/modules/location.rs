#![allow(dead_code)]
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

pub struct LocationTask {
    latitude: Option<f32>,
    longitude: Option<f32>,
}

impl ToJson for LocationTask {
    fn to_json (&self) -> Json {
        let mut map: BTreeMap<String, Json> = BTreeMap::new();
        map.insert("latitude".to_string(), self.latitude.to_json());
        map.insert("longitude".to_string(), self.longitude.to_json());
        map.to_json()
    }
 }

 impl LocationTask {
     pub fn set_latitude(&mut self, latitude: f32) {
         self.latitude = Some(latitude);
     }
     pub fn set_longitude(&mut self, longitude: f32) {
         self.longitude = Some(longitude);
     }
 }
