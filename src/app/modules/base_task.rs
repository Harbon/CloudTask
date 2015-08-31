#![allow(dead_code)]
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

pub trait BaseTask<T> {
    fn into_json () -> Option<Json>;
}
