use chrono::Utc;
use rusqlite::Connection;

use crate::{
    AppError,
    app::{App, Task},
    status::Status,
};

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

pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>, AppError> {
    let mut tasks = conn
        .prepare("SELECT * FROM tasks")
        .map_err(AppError::DbError)?;
    let tasks_rows = tasks
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .map_err(AppError::DbError)?;

    let mut tasks = vec![];

    for t in tasks_rows {
        let t = t.map_err(AppError::DbError)?;
        tasks.push(t);
    }

    Ok(tasks)
}
