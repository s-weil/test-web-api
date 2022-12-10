use actix_web::{get,post, web, App, HttpRequest, HttpResponse,Result, HttpServer};
use std::{thread, time};
use serde::Deserialize;
use serde::Serialize;
use actix_web::web::Json;


#[get("/ping")]
async fn ping(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[derive(Deserialize, Serialize)]
struct Item {
    name: String,
    description: Option<String>,
    price: f64,
    tax: Option<f64>
}

/// deserialize `Info` from request's body
#[post("/items/")]
async fn create_item(item: web::Json<Item>) -> Result<Json<Item>> {
    // Ok(format!("Welcome {}!", info.name))
    Ok(item)
}


#[get("/delayed/{time_ms}")]
async fn delayed(_: HttpRequest, time_ms: web::Path<u64>) -> HttpResponse {
    // TODO: randomize it; use async delay
    let ten_millis = time::Duration::from_millis(*time_ms.as_ref());
    thread::sleep(ten_millis);
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting on {}:{}", "0.0.0.0", 8080);

    HttpServer::new(|| App::new().service(ping).service(delayed).service(create_item))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
