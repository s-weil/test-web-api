use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::{thread, time};

#[get("/ping")]
async fn ping(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[get("/delayed")]
async fn delayed(_: HttpRequest) -> HttpResponse {
    // TODO: randomize it; use async delay
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting on {}:{}", "0.0.0.0", 8080);

    HttpServer::new(|| App::new().service(ping).service(delayed))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
