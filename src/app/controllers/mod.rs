#![allow(dead_code)]
use iron::status;
use iron::prelude::*;
use std::sync::Arc;
use bodyparser;
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
pub enum CloudTaskError{
    InternalServerError,
    InvalidParameter,
    UnknowError,
    InvalidToken,
    BadRequest,
    NotFound,
    ServerIsInMaintenance,
    ActionCrash,
    ThirdPartError,
}
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
    let data_array:Vec<Json> = match handle_request("Text", _req) {
        Ok(json_array) =>json_array,
        Err(err) => {
            match err {
                CloudTaskError::InternalServerError => {return Ok(Response::with((status::Ok, get_error(102, "Internal Server Error".to_string()).to_string())));},
                CloudTaskError::InvalidParameter => {
                    return Ok(Response::with((status::Ok, get_error(103, "Invalid Parameter".to_string()).to_string())));
                },
                    _ => {
                        return Ok(Response::with((status::Ok, get_error(1, "Unknown Errors".to_string()).to_string())));
                    }
                }
            }
        };

    let mut _num = "";
    match data_array.get(0) {
        Some(num) => {
            match num.as_string() {
                Some(num_str) => {
                    _num = num_str;
                },
                None => {
                    return Ok(Response::with((status::Ok, get_error(103, "Invalid Parameter".to_string()).to_string())));
                },
            }
        },
        None => {
            return Ok(Response::with((status::Ok, get_error(103, "Invalid Parameter".to_string()).to_string())));
        }
    }
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
    let json_array: Vec<Json> = match handle_request("ImageData", _req) {
        Ok(json_array) =>json_array,
        Err(err) => {
            match err {
                CloudTaskError::InternalServerError => {return Ok(Response::with((status::Ok, get_error(102, "Internal Server Error".to_string()).to_string())));},
                CloudTaskError::InvalidParameter => {
                    return Ok(Response::with((status::Ok, get_error(103, "Invalid Parameter".to_string()).to_string())));
                },
                    _ => {
                        return Ok(Response::with((status::Ok, get_error(1, "Unknown Errors".to_string()).to_string())));
                    }
                }
            }
    };
    let mut _base64_image_data = "";
    match json_array.get(0) {
        Some(json_base64_image_data) => {
            match json_base64_image_data.as_string() {
                Some(base64_image_data_string) => {
                    _base64_image_data = base64_image_data_string;
                },
                None => {
                    return Ok(Response::with((status::Ok, get_error(103, "Invalid Parameter".to_string()).to_string())));
                },
            }
        },
        None => {
            return Ok(Response::with((status::Ok, get_error(103, "Invalid Parameter".to_string()).to_string())));
        }
    }
    let dealed_data: Vec<&str>= _base64_image_data.split(|c| c == ':' || c == ';' || c == ',').collect();
    if dealed_data.len() != 4 {
        return Ok(Response::with((status::Ok, get_error
            (103, "Invalid Parameter".to_string()).to_string())));
    };
    let data_formate_info: Vec<&str> = dealed_data[1].split(|c| c == '/').collect();
    if !data_formate_info[0].eq("image") && !data_formate_info.len() == 2 {
        return Ok(Response::with((status::Ok, get_error
            (103, "Invalid Parameter".to_string()).to_string())));
    };
    let _image_buffer:Vec<u8>;
    let result_decode_base64 = dealed_data[3].from_base64();
    match result_decode_base64 {
        Ok(bytes) => _image_buffer = bytes,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (103, "Invalid Parameter".to_string()).to_string()))),
    }
    let file_name = "qrcode.".to_string() + data_formate_info[1];
    let mut f:File = match File::create(&file_name) {
        Ok(f) => f,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (102, "Internal Server Error".to_string()).to_string()))),
    };
    match f.write_all(&_image_buffer) {
        Ok(_) => (),
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (102, "Internal Server Error".to_string()).to_string()))),
    };
    match f.sync_all() {
        Ok(_) => (),
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (102, "Internal Server Error".to_string()).to_string()))),
    };
    match Command::new("sh").arg("decodeqr.sh").arg(&file_name).output() {
        Ok(_) => (),
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (102, "Internal Server Error".to_string()).to_string()))),
    };
    let output = match Command::new("cat").arg("qrcode.txt").output() {
        Ok(o) => o,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (102, "Internal Server Error".to_string()).to_string()))),
    };
    let mut result_full = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(_) => return Ok(Response::with((status::Ok, get_error
            (102, "Internal Server Error".to_string()).to_string()))),
    };
    result_full = result_full.replace("\n", "");
    let mut text_task = TextTask::new();
    text_task.set_text(result_full.trim_left_matches("QR-Code:").to_string());
    let data_array = vec![text_task.to_json()];
    let refer_express_task = Task {
        type_string: Some("Text".to_string()),
        data: Some(data_array.to_json()),
    };
    Ok(Response::with((status::Ok, refer_express_task.to_json().to_string())))
}

pub fn handle_request(data_type: &str, _req: &mut Request) -> Result<Vec<Json>, CloudTaskError> {
    let body = _req.get::<bodyparser::Json>();
    let json_body:Json;
    match body {
        Ok(Some(b)) => {json_body = b;},
        Ok(None) => {return Err(CloudTaskError::InvalidParameter);},
        Err(_) => {return Err(CloudTaskError::InvalidParameter);},
    };
    if json_body.is_object() {
        match json_body.find("type") {
            Some(json) => {
                    match json.as_string() {
                        Some(type_str) => {
                                if !type_str.eq(data_type) {
                                    return Err(CloudTaskError::InvalidParameter);
                                }
                            },
                        None => {return Err(CloudTaskError::InvalidParameter);},
                    }
                },
            None => {return Err(CloudTaskError::InvalidParameter);},
        };
        match json_body.find("data") {
            Some(json) => {
                if json.is_array() {
                    match json.as_array() {
                        Some(data_array) => {
                                return Ok(data_array.clone());
                            },
                        None => {return Err(CloudTaskError::InvalidParameter);},
                    }
                } else {
                    return Err(CloudTaskError::InvalidParameter);
                }
                },
            None => {return Err(CloudTaskError::InvalidParameter);},
        };

    } else {
        return Err(CloudTaskError::InvalidParameter);
    };


}
