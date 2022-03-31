use futures::TryStreamExt;
use sqlx::any::{AnyColumn, AnyConnection, AnyKind, AnyRow};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Column, Row, TypeInfo};

use std::error::Error;

pub async fn list_tables(link: &mut AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    match link.kind() {
        AnyKind::Postgres => list_tables_postgres(link).await,
        AnyKind::MySql => list_tables_mysql(link),
        AnyKind::Mssql => list_tables_mssql(link),
        AnyKind::Sqlite => list_tables_sqlite(link),
    }
}

pub async fn list_tables_postgres(link: &mut AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result: Vec<String> = vec![];
    let mut queried = sqlx::query(
        "SELECT table_name \
        FROM information_schema.tables \
        WHERE table_type='BASE TABLE' \
        AND table_schema='public'",
    )
    .fetch(link);
    while let Some(row) = queried.try_next().await? {
        let table = row.try_get::<String, _>(0)?;
        result.push(table);
    }
    Ok(result)
}

pub fn list_tables_mysql(_link: &mut AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    Err(liz::liz_debug::throw("Unimplemented function.".into()))
}

pub fn list_tables_mssql(_link: &mut AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    Err(liz::liz_debug::throw("Unimplemented function.".into()))
}

pub fn list_tables_sqlite(_link: &mut AnyConnection) -> Result<Vec<String>, Box<dyn Error>> {
    Err(liz::liz_debug::throw("Unimplemented function.".into()))
}

pub fn csv_value(row: &AnyRow, index: usize, column: &AnyColumn) -> Result<String, sqlx::Error> {
    if column.type_info().is_null() || column.type_info().is_void() {
        return Ok(String::default());
    }
    let column_type = column.type_info().name();
    let value = match column_type {
        "VARCHAR" => {
            let value: Result<Option<String>, _> = row.try_get(index);
            match value {
                Ok(value) => {
                    if let Some(value) = value {
                        value
                    } else {
                        String::default()
                    }
                }
                Err(erro) => format!("Value error: {:?}", erro),
            }
        }
        "DATE" => {
            let value: Result<Option<DateTime<Utc>>, _> = row.try_get(index);
            match value {
                Ok(value) => {
                    if let Some(value) = value {
                        liz::liz_times::fmt_ad(&value)
                    } else {
                        String::default()
                    }
                }
                Err(erro) => format!("Value error: {:?}", erro),
            }
        }
        unknown => {
            format!("Type error: {}", unknown)
        }
    };
    Ok(csv_fix(value))
}

pub fn csv_fix(value: String) -> String {
    let mut result = value
        .replace('"', "\"\"")
        .replace('\\', "\\\\")
        .replace("\r", "\\r")
        .replace("\n", "\\n")
        .replace("\t", "\\t");
    if result.contains('"') || result.contains(",") {
        result.insert(0, '"');
        result.push('"');
    }
    result
}
