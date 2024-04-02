pub mod models;
pub mod schema;
pub mod database;
mod todos;


use actix_web::{get, HttpResponse, post, web, Responder};

use lazy_static::lazy_static;
use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
struct FormItem {
    item: Vec<String>
}
#[post("/reorder")]
pub async fn reorder(new_orders: web::Json<FormItem>) -> impl Responder {
    let new_orders_i32: Vec<i32> = new_orders.item.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let result = todos::reorder_todo(new_orders_i32);

    let mut context = tera::Context::new();
    context.insert("list", &result);

    let page_content = TEMPLATES.render("list.html", &context).unwrap();
    println!("{:?}", result);



    HttpResponse::Ok().body(page_content)
}

#[get("/")]
pub async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    let res = todos::get_todos();
    println!("{:?}", res);
    context.insert("list", &res);
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}
