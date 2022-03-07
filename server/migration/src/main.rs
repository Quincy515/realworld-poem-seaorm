use sea_schema::migration::*;

use migration::Migrator;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
