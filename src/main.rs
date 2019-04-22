#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io;

use rocket::response::NamedFile;
use rocket::http::RawStr;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
