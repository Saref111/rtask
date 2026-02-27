use rusqlite::Connection;

use crate::AppError;

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
