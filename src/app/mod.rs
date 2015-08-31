#![allow(dead_code)]
#![allow(unused_imports)]
pub mod routes;
pub mod controllers;
pub mod modules;
use router::Router;
use iron::prelude::*;

pub fn run () {
    println!("start run app");
    Iron::new(routes::init_router()).http("localhost:4949").unwrap();
    println!("stop run app");
}