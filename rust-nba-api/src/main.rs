use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || App::new().service(hello))
    .bind(("localhost", 5001))?
    .run()
    .await
}
