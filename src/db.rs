use chrono::Utc;
use rusqlite::Connection;

use crate::{AppError, status::Status};

pub fn create_bd() -> Result<Connection, AppError> {
    let conn = Connection::open("./rtask.db").map_err(AppError::DbError)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        status TEXT NOT NULL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL
    )",
        (),
    )
    .map_err(AppError::DbError)?;

    Ok(conn)
}

pub fn create_task(conn: &Connection, title: String) -> Result<(), AppError> {
    let now = Utc::now().timestamp().to_string();
    conn.execute(
        "INSERT INTO tasks (title, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        [title, Status::ToDo.into(), now.to_owned(), now.to_owned()],
    )
    .map_err(AppError::DbError)?;
    Ok(())
}
