pub mod db;
pub mod config;
pub mod data_structs;
mod error_enums;

use db::models::board::BoardModel;
use data_structs::model_structs::board::CreateBoardData;
use config::database;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the DB
    let db = database::connect().await;

    // Create BoardModel instance
    let board_model = BoardModel { db };

    // Example: create a board
    let new_board = board_model
        .create_board(CreateBoardData {
            name: "My First Board".to_string(),
            created_by: 1,
        })
        .await?;

    println!("Created board with id: {}", new_board.id);

    Ok("Board Creation successful!")
}
