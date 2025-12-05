use sea_orm::{
    prelude::*,
    ActiveModelTrait, DatabaseTransaction, EntityTrait, Set,
};
use chrono::Utc;
use error_enums::db_errors::DbErrors;
use data_structs::model_structs::board::CreateBoardData;

pub struct BoardModel; 

impl BoardModel {
    //TODO: Add default categories to board creation method
    pub async fn create_board(db: &DatabaseConnection, creator_data: CreateBoardData,) -> Result<board::Model, DbErrors> {
        // Build ActiveModel
        let new_board = board::ActiveModel {
            name: Set(creator_data.name.clone()),
            created_by: Set(creator_data.created_by),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default() // fills id etc.
        };
    
        // Insert into DB
        let board = new_board.insert(db).await?;
    
        // Return the inserted board
        Ok(board)
    }

    pub async fn get_board(&self, id: i32) -> Result<Board, Error> {
        let board = Board::find_by_id(id).one(db).await?;
        Ok(board)
    }

    pub async fn update_board(&self, id: i32, board: Board) -> Result<Board, Error> {
        let board = Board::update(board).filter(Column::Id.eq(id)).exec(db).await?;
        Ok(board)
    }

    pub async fn delete_board(&self, id: i32) -> Result<(), Error> {
        let board = Board::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub fn list_boards(&self) -> Result<Vec<Board>, Error> {
        let boards = Board::find().all(db).await?;
        Ok(boards)
    }
}
