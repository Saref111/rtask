mod app;
mod db;

use std::io::Error;

use app::App;
use rusqlite::Error as DBError;

use crate::db::create_bd;

#[derive(Debug)]
enum AppError {
    DbError(DBError),
    IOError(Error),
}

fn main() -> Result<(), AppError> {
    let conn = create_bd()?;
    ratatui::run(|terminal| App::new(conn).run(terminal)).map_err(AppError::IOError)
}
