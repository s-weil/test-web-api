use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[get("/")]
async fn health(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting on {}:{}", "0.0.0.0", 8000);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()    
    .await

}
