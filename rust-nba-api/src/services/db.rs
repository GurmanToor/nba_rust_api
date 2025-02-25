use std::env;

use chrono::Utc;
use futures_util::StreamExt;
use mongodb::{bson::{doc, from_document, oid::ObjectId, DateTime}, error::Error, results::{InsertOneResult, UpdateResult}, Client, Collection};
use crate::models::{game_model::{FullGame, Game}, player_model::Player, team_model::Team};

pub struct Database {
    game: Collection<Game>,
    player: Collection<Player>,
    team: Collection<Team>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGODB_URI") {
            Ok(uri) => uri.to_string(),
            Err(_) => "mongodb://localhost:27017/?directConnection=true".to_string(),
        };

        let client = Client::with_uri_str(&uri)
            .await
            .unwrap();
        let db = client.database("nba");

        let game: Collection<Game> = db.collection("game");
        let player: Collection<Player> = db.collection("player");
        let team: Collection<Team> = db.collection("team");

        Database { game, player, team }
    }

    pub async fn create_game(&self, game: Game) -> Result<InsertOneResult, Error> {
        let result = self.game.insert_one(game)
            .await
            .ok()
            .expect("Error creating game");

        Ok(result)
    }

    pub async fn create_player(&self, player: Player) -> Result<InsertOneResult, Error> {
        let result = self.player.insert_one(player)
            .await
            .ok()
            .expect("Error creating player");

        Ok(result)
    }

    pub async fn create_team(&self, team: Team) -> Result<InsertOneResult, Error> {
        let result = self.team.insert_one(team)
            .await
            .ok()
            .expect("Error creating team");

        Ok(result)
    }

    pub async fn complete_game(&self, game_id: &str) -> Result<UpdateResult, Error> {
        let result = self.game.update_one(
            doc! { "_id": ObjectId::parse_str(game_id).expect("Failed to parse game id") },
            doc! { "$set": doc! { "completed": true } }
        ).await
        .ok()
        .expect("Error completing game");

        Ok(result)
    }

    pub async fn get_live_games(&self) -> Result<Vec<FullGame>, Error> {

        let mut results = self
            .game
            .aggregate(vec![
                doc! {
                    "$match": {
                        "completed": false,
                        "start_time": {
                            "$gte": DateTime::from_system_time(Utc::now().into())
                        }
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "team",
                        "localField": "home_team",
                        "foreignField": "_id",
                        "as": "home_team"
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "team",
                        "localField": "away_team",
                        "foreignField": "_id",
                        "as": "away_team"
                    }
                },
                doc! {
                    "$unwind": doc! {
                        "path": "$home_team"
                    }
                },
                doc! {
                    "$unwind": doc! {
                        "path": "$away_team"
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "player",
                        "localField": "home_team._id",
                        "foreignField": "team_id",
                        "as": "home_players"
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "player",
                        "localField": "away_team._id",
                        "foreignField": "team_id",
                        "as": "away_players"
                    }
                },
            ])
            .await
            .ok()
            .expect("Error getting live games");

        let mut games: Vec<FullGame> = Vec::new();

        while let Some(result) = results.next().await {
            match result {
                Ok(doc) => {
                    let game: FullGame = from_document(doc)
                        .expect("Error converting document to full game");

                    games.push(game);
                },
                Err(e) => {
                    panic!("Error getting live games: {}", e);
                }
            }
        }
        Ok(games)
    }
}