#![allow(dead_code)]
use iron::status;
use iron::prelude::*;
use std::sync::Arc;
use urlencoded::UrlEncodedQuery;
use urlencoded::UrlEncodedBody;
use std::collections::HashMap;
use app::modules::text::TextTask;
use app::modules::image::ImageTask;
use app::modules::location::LocationTask;
use app::modules::webpage::WebpageTask;
use app::modules::error::Error;
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::process::Command;
use rustc_serialize::base64::FromBase64;
use std::path::Path;
use image::png::PNGEncoder;
use image;
use std::fs::File;
use std::io::Write;
use app::helpers;

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

pub fn get_error(code: i32, message: String) -> Json {
    let mut error = Error::new();
    error.set_code(code);
    error.set_message(message);
    let data_array = vec!(error.to_json());
    let error_task = Task {
        type_string: Some("Error".to_string()),
        data: Some(data_array.to_json()),
    };
    error_task.to_json()
}


/*
    查快递任务处理
*/
pub fn handle_refer_express(_req: &mut Request) ->IronResult<Response> {
    let query_map: &HashMap<String, Vec<String>> = match _req.get_ref::<UrlEncodedQuery>() {
        Ok(map) => map,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    let array_num: &Vec<String> = match query_map.get("num") {
        Some(array) => array,
        None => return Ok(Response::with((status::Ok, get_error
            (103, "INTERNAL JSON".to_string()).to_string()))),
    };
    let _num: &str = match array_num.get(0) {
        Some(s) => s,
        None => return Ok(Response::with((status::Ok, get_error
            (103, "INTERNAL JSON".to_string()).to_string()))),
    };
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

pub fn handle_decode_qr(_req: &mut Request) -> IronResult<Response> {
    let query_map: &HashMap<String, Vec<String>> = match _req.get_ref::<UrlEncodedBody>() {
        Ok(map) => map,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    let array_type: &Vec<String> = match query_map.get("type") {
        Some(array) => array,
        None => return Ok(Response::with((status::Ok, get_error
            (103, "INVALID JSON".to_string()).to_string()))),
    };
    let _type = match array_type.get(0) {
        Some(t) => t,
        None => return Ok(Response::with((status::Ok, get_error
            (500, "INVALID JSON".to_string()).to_string()))),
    };
    let array_data:&Vec<String> = match query_map.get("data") {
        Some(array) => array,
        None => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    let _data: &str = match array_data.get(0) {
        Some(data) => data,
        None => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    if !_data.starts_with("data:") {
        return Ok(Response::with((status::Ok, get_error
            (103, "Not A BASE64 ENCODE IMAGE DATA".to_string()).to_string())));
    };
    let dealed_data: Vec<&str>= _data.split(|c| c == ':' || c == ';' || c == ',').collect();
    if dealed_data.len() != 4 {
        return Ok(Response::with((status::Ok, get_error
            (103, "Not A BASE64 ENCODE IMAGE DATA".to_string()).to_string())));
    };
    let data_formate_info: Vec<&str> = dealed_data[1].split(|c| c == '/').collect();
    if !data_formate_info[0].eq("image") && !data_formate_info.len() == 2 {
        return Ok(Response::with((status::Ok, get_error
            (103, "Not A BASE64 ENCODE IMAGE DATA".to_string()).to_string())));
    };
    let _image_buffer:Vec<u8>;
    let result_decode_base64 = dealed_data[3].from_base64();
    match result_decode_base64 {
        Ok(bytes) => _image_buffer = bytes,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (103, "Not A BASE64 ENCODE IMAGE DATA".to_string()).to_string()))),
    }
    let file_name = "qrcode.".to_string() + data_formate_info[1];
    let mut f:File = match File::create(&file_name) {
        Ok(f) => f,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    match f.write_all(&_image_buffer) {
        Ok(_) => (),
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    match f.sync_all() {
        Ok(_) => (),
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    match Command::new("sh").arg("decodeqr.sh").arg(&file_name).output() {
        Ok(_) => (),
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    let output = match Command::new("cat").arg("qrcode.txt").output() {
        Ok(o) => o,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    let result_full = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (500, "INTERNAL ERROR".to_string()).to_string()))),
    };
    let mut text_task = TextTask::new();
    let is_qrcode_info = result_full.contains(':');
    if !is_qrcode_info {
            return Ok(Response::with((status::Ok, get_error
                (103, "NOT A QRCODE IMAGE".to_string()).to_string())));
    }
    let result_qr:Vec<&str> = result_full.split(|c|c == ':').collect();
    text_task.set_text(result_qr[1].to_string());
    let data_array = vec![text_task.to_json()];
    let refer_express_task = Task {
        type_string: Some("Text".to_string()),
        data: Some(data_array.to_json()),
    };
    Ok(Response::with((status::Ok, refer_express_task.to_json().to_string())))
}
