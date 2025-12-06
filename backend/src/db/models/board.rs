use sea_orm::{prelude::*, ActiveModelTrait, DatabaseTransaction, EntityTrait, Set};
use chrono::Utc;
use error_enums::db_errors::DbErrors;
use data_structs::model_structs::board::CreateBoardData;

pub struct BoardModel{
    pub db: DatabaseConnection,
}; 

 impl BoardModel {

    pub async fn create_board(&self, creator_data: CreateBoardData,) -> Result<board::Model, DbErrors> {
        //start transaction to improve atomicity
        let txn = self.db.begin().await?;

        // Build ActiveModel
        let new_board = boards::ActiveModel {
            name: Set(creator_data.name),
            created_by: Set(creator_data.created_by),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default() // fills id etc.
        };
    
        let board = new_board.insert(&txn).await?;

        const DEFAULT_CATEGORIES: [(&str, &str, bool); 5] = [
            ("Backlog", "#94A3B8", false),
            ("Todo", "#3B82F6", false),
            ("In Progress", "#F59E0B", false),
            ("Done", "#10B981", true),
            ("Cancelled", "#EF4444", false)
        ];

        // build active models for default categories
        // creates a vector of active models for each category
        let category_models: Vec<categories::ActiveModel> = DEFAULT_CATEGORIES
            .iter()
            .enumerate()
            .map(|(position, (name, color, is_done))| categories::ActiveModel {
                board_id: Set(board.id),
                name: Set(name.to_string()),
                color: Set(color.to_string()),
                position: Set(position as i32),
                is_done: Set(*is_done),
                ..Default::default()
            })
            .collect(); 
            
            // insert default categories into database as batch insert
        categories::Entity::insert_many(category_models) // insert many does automatic batch inserts
            .exec(&txn)
            .await?

        txn.commit().await?;

        Ok(board)
    }

}

    /*
    TODO List for creating these methods:
    create an update_board method
    create a delete_board method
    create a get_board method
    create a list_boards method

    use the boards.ts in Axxon for ref
    */
