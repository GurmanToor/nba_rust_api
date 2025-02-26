use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub team_id: ObjectId,
    pub position: String,
    pub height_in_cm: u16,
    pub weight_in_kg: u16,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRequest {
    pub first_name: String,
    pub last_name: String,
    pub team_id: String,
    pub position: String,
    pub height_in_cm: u16,
    pub weight_in_kg: u16,
    pub active: bool,
}

impl TryFrom<PlayerRequest> for Player {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: PlayerRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            first_name: item.first_name,
            last_name: item.last_name,
            team_id: ObjectId::parse_str(&item.team_id).expect("Failed to parse team id"),
            position: item.position,
            height_in_cm: item.height_in_cm,
            weight_in_kg: item.weight_in_kg,
            active: item.active,
        })
    }
}
