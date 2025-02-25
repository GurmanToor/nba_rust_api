use std::time::SystemTime;

use chrono::Utc;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::{player_model::Player, team_model::Team};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub _id: ObjectId,
    pub away_team: ObjectId,
    pub home_team: ObjectId,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub completed: bool,
    pub home_score: u8,
    pub away_score: u8,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GameRequest {
    pub away_team: String,
    pub home_team: String,
    pub start_time: String,
    pub duration_in_minutes: u8,
    pub home_score: u8,
    pub away_score: u8
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FullGame {
    pub _id: ObjectId,
    pub away_team: Team,
    pub home_team: Team,
    pub home_players: Vec<Player>,
    pub away_players: Vec<Player>,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub completed: bool,
    pub home_score: u8,
    pub away_score: u8
}

impl TryFrom<GameRequest> for Game {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: GameRequest) -> Result<Self, Self::Error> {
        let chrono_start_time: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.start_time)
            .map_err(|err| format!("Failed to parse start time: {}", err))?
            .with_timezone(&Utc)
            .into();

        Ok(Self {
            _id: ObjectId::new(),
            away_team: ObjectId::parse_str(&item.away_team).expect("Failed to parse away team id"),
            home_team: ObjectId::parse_str(&item.home_team).expect("Failed to parse home team id"),
            start_time: DateTime::from(chrono_start_time),
            duration_in_minutes: item.duration_in_minutes,
            completed: false,
            home_score: item.home_score,
            away_score: item.away_score,
        })
    }
}