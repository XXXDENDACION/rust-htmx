pub mod models;
pub mod schema;
pub mod database;

use std::io::prelude::*;

use actix_web::{get, HttpResponse, post, Responder};

use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

#[post("/click")]
pub async fn click() -> impl Responder {
    HttpResponse::Ok().body("Clicked!@")
}

#[get("/")]
pub async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("message_from_rust", &[0,1,2,3,4]);
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}
