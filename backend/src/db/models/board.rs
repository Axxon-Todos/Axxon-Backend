use sea_orm::entity::*;
use sea_orm::DatabaseConnection;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;

pub struct BoardModel; 

impl BoardModel {

    pub async fn create_board(
        db: &DatabaseConnection,
        name: &str,
        created_by: i32,
        member_emails: Option<Vec<String>>,
    ) -> Result<board::Model, sea_orm::DbErr> {
        // Step 1: Build the ActiveModel
        let new_board = board::ActiveModel {
            name: Set(name.to_owned()),
            created_by: Set(created_by),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default() // auto-fill optional fields like id
        };
    
        // Step 2: Insert into DB
        let board = new_board.insert(db).await?;
    
        // Step 3: Return the inserted board
        Ok(board)
    }

    pub async fn create_board(&self, board: Board) -> Result<Board, Error> {
        let board = Board::create(board).exec(db).await?;
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
