use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Categories::Table)
                    .modify_column(
                        ColumnDef::new(Categories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .modify_column(
                        ColumnDef::new(Categories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .modify_column(
                        ColumnDef::new(Categories::IsDone)
                            .boolean()
                            .not_null()
                            .default(false)
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Categories::Table)
                    .modify_column(
                        ColumnDef::new(Categories::CreatedAt)
                            .timestamp_with_time_zone()
                            .null()
                    )
                    .modify_column(
                        ColumnDef::new(Categories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null()
                    )
                    .modify_column(
                        ColumnDef::new(Categories::IsDone)
                            .boolean()
                            .null()
                    )
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    BoardId,
    Name,
    Color,
    Position,
    IsDone,
    CreatedAt,
    UpdatedAt,
}