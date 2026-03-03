use std::{error::Error, fmt};

use rusqlite::types::{FromSql, FromSqlError, ValueRef};

#[derive(Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.split(" ").collect::<String>().to_lowercase();
        match value.as_str() {
            "todo" => Ok(Self::ToDo),
            "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(String::from("wrong status")),
        }
    }

    type Error = String;
}

impl Into<String> for Status {
    fn into(self) -> String {
        match self {
            Self::Done => "Done".to_string(),
            Self::InProgress => "In progress".to_string(),
            Self::ToDo => "To do".to_string(),
        }
    }
}

impl FromSql for Status {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(text) => {
                let s = std::str::from_utf8(text).map_err(|_| FromSqlError::InvalidType)?;
                match s {
                    "Done" => Ok(Status::Done),
                    "In progress" => Ok(Status::InProgress),
                    "To do" => Ok(Status::ToDo),
                    other => Err(FromSqlError::Other(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("unexpected status string `{}`", other),
                    )))),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
