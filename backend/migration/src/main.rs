use migration::{Migrator};
use sea_orm_migration::cli;
use tokio;

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        // create a Migrator instance and pass it in
        cli::run_cli(Migrator).await;
    });
}
