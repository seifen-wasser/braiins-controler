mod device;
mod api;
mod utils;


use actix_web::{web, App, HttpServer};
use std::sync::Mutex;


#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let test = "192.123.123.40";
    let addr = format!("http://{}:50051", test);

    //let auth_token = device::braiins::login(&addr, "root", "test").await.unwrap();

    HttpServer::new(|| {
        App::new()
            .configure(api::routs::init_routs)
            .route("/", web::get().to(|| async { "Hello, World!" }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}