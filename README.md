# NBA Rust API

This is a Rust-based API for managing NBA-related data, including games, players, and teams. The API is built using the Actix-web framework and MongoDB for data storage.

## Models

- `Game`: Represents a game with fields like `_id`, `away_team`, `home_team`, `start_time`, `duration_in_minutes`, `completed`, `home_score`, and `away_score`.
- `GameRequest`: Represents a request to create a game with fields like `away_team`, `home_team`, `start_time`, `duration_in_minutes`, `home_score`, and `away_score`.
- `FullGame`: Represents a full game with additional details like `away_team`, `home_team`, `home_players`, and `away_players`.

## Routes

- `game_route.rs`: Contains routes for creating, completing, and retrieving games.
- `player_route.rs`: Contains routes for creating players.
- `team_route.rs`: Contains routes for creating teams.

## Services

- `db.rs`: Contains the database initialization and connection logic.

## Getting Started

### Prerequisites

- Rust
- MongoDB

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/nba_rust_api.git
    cd nba_rust_api
    ```

2. Install dependencies:
    ```sh
    cargo build
    ```

3. Run the server:
    ```sh
    cargo run
    ```

The server will start on `localhost:5001`.

## API Endpoints

- `GET /`: Returns "Hello world!"
- `POST /create_game`: Creates a new game.
- `POST /create_player`: Creates a new player.
- `POST /create_team`: Creates a new team.
- `POST /complete_game`: Completes a game.
- `GET /get_games`: Retrieves all games.

## License

This project is licensed under the MIT License.
