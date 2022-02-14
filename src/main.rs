use sqlx::{AnyConnection, Connection};
use std::error::Error;

mod clip;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = clip::parse();
    let base = args
        .value_of("base")
        .expect("You must inform the base URL connection string.");
    if let Some(dir) = args.value_of("export_to_csv") {
        export_to_csv(base, dir).await?;
    }
    Ok(())
}

async fn connect(base: &str) -> Result<AnyConnection, Box<dyn Error>> {
    Ok(AnyConnection::connect(base).await?)
}

async fn export_to_csv(base: &str, _dir: &str) -> Result<(), Box<dyn Error>> {
    let link = connect(base).await?;
    let _tables = utils::list_tables(&link)?;
    Ok(())
}
