use actix_web::{get, post, put, web::{Data, Json, Path}, HttpResponse};
use crate::{models::game_model::{Game, GameRequest}, services::db::Database};

#[post("/game")]
pub async fn create_game(db: Data<Database>, request: Json<GameRequest>) -> HttpResponse {
    match db
        .create_game(
            Game::try_from(GameRequest {
                home_team: request.home_team.clone(),
                away_team: request.away_team.clone(),
                start_time: request.start_time.clone(),
                duration_in_minutes: request.duration_in_minutes.clone(),
                home_score: request.home_score.clone(),
                away_score: request.away_score.clone(),
            })
            .expect("Error converting GameRequest to Game")
        ).await {
            Ok(game) => HttpResponse::Ok().json(game),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
}

#[get("/games")]
pub async fn get_games(db: Data<Database>) -> HttpResponse {
    match db.get_live_games().await {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/game/{id}/complete")]
pub async fn complete_game(db: Data<Database>, path: Path<(String,)>) -> HttpResponse {
    let id = path.into_inner().0;
    match db.complete_game(id.as_str()).await {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}