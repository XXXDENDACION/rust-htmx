use actix_web::{App, HttpServer};
use actix_files as fs;
use rusthtmx::{index, click, reorder};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/assets", "./assets"))
            .service(fs::Files::new("/assets", "./assets/web/sortable.js"))
            .service(fs::Files::new("/styles", "./styles"))
            .service(click)
            .service(reorder)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}