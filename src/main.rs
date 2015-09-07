extern crate iron;
extern crate urlencoded;
extern crate router;
extern crate rustc_serialize;
extern crate image;

mod app;
fn main() {
    app::run();
}
