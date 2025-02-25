use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub _id: ObjectId,
    pub name: String,
    pub city: String,
    pub abbreviation: String,
}

pub struct TeamRequest {
    pub name: String,
    pub city: String,
    pub abbreviation: String,
}

impl TryFrom<TeamRequest> for Team {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: TeamRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            city: item.city,
            abbreviation: item.abbreviation,
        })
    }
}