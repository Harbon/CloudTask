#![allow(dead_code)]
use iron::status;
use iron::prelude::*;
use std::sync::Arc;
use urlencoded::UrlEncodedQuery;
use std::collections::HashMap;
use app::modules::text::TextTask;
use app::modules::image::ImageTask;
use app::modules::location::LocationTask;
use app::modules::webpage::WebpageTask;
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

pub struct Task {
    type_string: Option<String>,
    data: Option<Json>,
}

impl ToJson for Task {
    fn to_json(&self) -> Json {
        let mut map: BTreeMap<String, Json> = BTreeMap::new();
        map.insert("type".to_string(), self.type_string.to_json());
        map.insert("data".to_string(), self.data.to_json());
        Json::Object(map)
    }
}



/*
    查快递任务处理
*/
pub fn handle_refer_express(_req: &mut Request) ->IronResult<Response> {
    let query_map = _req.get_ref::<UrlEncodedQuery>().unwrap();
    let _num = query_map.get("num").unwrap().get(0).unwrap();
    let _url = "http://m.kuaidi100.com/result.jsp?nu=".to_string() + _num;
    let mut web_page_task = WebpageTask::new();
    web_page_task.set_url(_url);
    let data_array = vec![web_page_task.to_json()];
    let refer_express_task = Task {
        type_string: Some("WebPage".to_string()),
        data: Some(data_array.to_json()),
    };
    Ok(Response::with((status::Ok, refer_express_task.to_json().to_string())))
}
