extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate image;
extern crate bodyparser;
extern crate persistent;
extern crate serde;
extern crate serde_json;
mod app;
fn main() {
    app::run();
}
