#![allow(dead_code)]
#![allow(unused_imports)]
pub mod routes;
pub mod controllers;
pub mod modules;
pub mod helpers;
use router::Router;
use iron::prelude::*;
use persistent::Read;
use bodyparser;
const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;
pub fn run () {
    println!("start run app");
    // Iron::new(routes::init_router()).http("121.40.132.139:4949").unwrap();
    let mut chain = Chain::new(routes::init_router());
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:4949").unwrap();
    println!("stop run app");
}
