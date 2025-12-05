use sea_orm::entity::*;
use sea_orm::DatabaseConnection;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;

pub struct BoardModel; 

impl BoardModel {
    pub fn create_board(&self, board: Board) -> Result<Board, Error> {
        let board = Board::create(board).exec(db).await?;
        Ok(board)
    }

    pub fn get_board(&self, id: i32) -> Result<Board, Error> {
        let board = Board::find_by_id(id).one(db).await?;
        Ok(board)
    }

    pub fn update_board(&self, id: i32, board: Board) -> Result<Board, Error> {
        let board = Board::update(board).filter(Column::Id.eq(id)).exec(db).await?;
        Ok(board)
    }

    pub fn delete_board(&self, id: i32) -> Result<(), Error> {
        let board = Board::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub fn list_boards(&self) -> Result<Vec<Board>, Error> {
        let boards = Board::find().all(db).await?;
        Ok(boards)
    }
}
