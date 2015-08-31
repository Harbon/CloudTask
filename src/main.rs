extern crate iron;
extern crate urlencoded;
extern crate router;
extern crate rustc_serialize;
mod app;
fn main() {
    app::run();
}
