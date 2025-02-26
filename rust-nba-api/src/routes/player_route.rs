use actix_web::{post, web::{Data, Json}, HttpResponse};

use crate::{models::player_model::{Player, PlayerRequest}, services::db::Database};

#[post("/player")]
pub async fn create_player(db: Data<Database>, request: Json<PlayerRequest>) -> HttpResponse {
    match db
        .create_player(
            Player::try_from(PlayerRequest {
                first_name: request.first_name.clone(),
                last_name: request.last_name.clone(),
                team_id: request.team_id.clone(),
                position: request.position.clone(),
                height_in_cm: request.height_in_cm.clone(),
                weight_in_kg: request.weight_in_kg.clone(),
                active: request.active.clone(),
            })
            .expect("Error converting PlayerRequest to Player")
        ).await {
            Ok(player) => HttpResponse::Ok().json(player),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
}