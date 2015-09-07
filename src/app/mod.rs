#![allow(dead_code)]
#![allow(unused_imports)]
pub mod routes;
pub mod controllers;
pub mod modules;
pub mod helpers;
use router::Router;
use iron::prelude::*;

pub fn run () {
    println!("start run app");
    Iron::new(routes::init_router()).http("121.40.132.139:4949").unwrap();
    println!("stop run app");
}
