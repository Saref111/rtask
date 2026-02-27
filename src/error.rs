use std::io::Error as IOError;

use rusqlite::Error as DBError;

#[derive(Debug)]
pub enum AppError {
    DbError(DBError),
    IOError(IOError),
}
