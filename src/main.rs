#![allow(warnings)]

use ntex::web;

#[web::get("/hello")]
async fn hello() -> web::HttpResponse {
    web::HttpResponse::Ok().body("Hello this is a simple example! rust ntex")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
