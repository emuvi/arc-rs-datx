use std::error::Error;

mod clip;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = clip::parse();
    let base = args
        .value_of("base")
        .expect("You must inform the base URL connection string.");
    if let Some(dir) = args.value_of("export-to-csv") {
        datx::export_to_csv(base, dir).await?;
    }
    Ok(())
}


