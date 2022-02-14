use sqlx::any::AnyKind;
use sqlx::AnyConnection;

use std::error::Error;

pub fn list_tables(link: &AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    match link.kind() {
        AnyKind::Postgres => list_tables_postgres(link),
        AnyKind::MySql => list_tables_mysql(link),
        AnyKind::Mssql => list_tables_mssql(link),
        AnyKind::Sqlite => list_tables_sqlite(link),
    }
}

pub fn list_tables_postgres(_link: &AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(Vec::new())
}

pub fn list_tables_mysql(_link: &AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(Vec::new())
}

pub fn list_tables_mssql(_link: &AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(Vec::new())
}

pub fn list_tables_sqlite(_link: &AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(Vec::new())
}
