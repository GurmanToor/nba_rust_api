mod models;
mod services;
mod routes;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::{game_route::{complete_game, create_game, get_games}, player_route::create_player, team_route::create_team};
use services::db::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || App::new().app_data(db_data.clone())
        .service(hello)
        .service(create_game)
        .service(create_player)
        .service(create_team)
        .service(complete_game)
        .service(get_games))
    .bind(("localhost", 5001))?
    .run()
    .await
}
