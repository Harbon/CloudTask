#![allow(dead_code)]
use app::controllers;
use router::Router;

pub fn init_router() -> Router {
    let mut router = Router::new();
    router.get("/referExpress", controllers::handle_refer_express);
    router
}
