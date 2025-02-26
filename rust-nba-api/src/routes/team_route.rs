use actix_web::{post, web::{Data, Json}, HttpResponse};
use crate::{models::team_model::{Team, TeamRequest}, services::db::Database};

#[post("/team")]
pub async fn create_team(db: Data<Database>, request: Json<TeamRequest>) -> HttpResponse {
    match db
        .create_team(
            Team::try_from(TeamRequest {
                name: request.name.clone(),
                city: request.city.clone(),
                abbreviation: request.abbreviation.clone(),
            })
            .expect("Error converting TeamRequest to Team")
        ).await {
            Ok(team) => HttpResponse::Ok().json(team),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
}