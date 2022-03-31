use futures::TryStreamExt;
use sqlx::{Row, Column};
use sqlx::{AnyConnection, Connection};

use std::error::Error;
use std::path::{Path, PathBuf};

mod utils;

pub async fn connect(base: &str) -> Result<AnyConnection, Box<dyn Error>> {
    Ok(AnyConnection::connect(base).await?)
}

pub async fn export_to_csv(base: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    let dir = Path::new(dir);
    std::fs::create_dir_all(dir)?;
    let mut link = connect(base).await?;
    let tables = utils::list_tables(&mut link).await?;
    for table in tables {
        let destiny = dir.join(format!("{}.csv", table));
        export_table_to_csv(&mut link, table, destiny).await?;
    }
    Ok(())
}

pub async fn export_table_to_csv(
    link: &mut AnyConnection,
    table: String,
    destiny: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut body = String::new();
    let query = format!("SELECT * FROM {}", table);
    let mut queried = sqlx::query(&query).fetch(link);
    let mut first = true;
    while let Some(row) = queried.try_next().await? {
        let columns = row.columns();
        if first {
            for i in 0..columns.len() {
                if i > 0 {
                    body.push(',');
                }
                body.push_str(columns[i].name());
            }
            body.push('\n');
            first = false;
        }
        for i in 0..columns.len() {
            let value = utils::csv_value(&row, i, &columns[i])?;
            if i > 0 {
                body.push(',');
            }
            body.push_str(&value);
        }
        body.push('\n');
    }
    std::fs::write(destiny, body)?;
    Ok(())
}
