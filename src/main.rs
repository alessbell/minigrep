#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]
// https://rocket.rs/v0.4/guide/requests/#optional-parameters
extern crate env_logger;
// #[macro_use] extern crate log;
#[macro_use] extern crate rocket;
use rocket::http::RawStr;
// use url::Url;

#[get("/?<board>")]
fn hello(board: Option<&RawStr>) -> String {
    println!("{:?}", board);
    board.map(|board| format!("Hi, {:?}!", board))
        .unwrap_or_else(|| "Hello!".into())
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
