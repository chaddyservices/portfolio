#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io;

use rocket::response::NamedFile;
use rocket::http::RawStr;

/**
 * FILENAMES:
 * GET:
 * index.html       -> /
 * personalize.html -> /personalize
 *
 * POST:
 * Order            -> /order/<package>
 * Personalize      -> /personalize/<reviews><html><css><js><backend><pages>
*/

// GETS

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/personalize")]
fn personalize() -> io::Result<NamedFile> {
    NamedFile::open("public/personalize.html")
}

#[get("/personalize/<reviews>/<html>/<css>/<js>/<backend>/<pages>")]
fn personalize_order(reviews: u8, html: bool, css: bool, js: bool, backend: bool, pages: u8) -> String {
    String::from(format!("Following information has been sent:\nReviews: {}\nHTML: {}\nCSS: {}\nJavaScript: {}\nBackend: {}\nPages: {}", reviews, html, css, js, backend, pages))
}

fn main() {
    rocket::ignite().mount("/", routes![index])
                    .mount("/personalize", routes![personalize])            
                    .launch();
}
