mod app;
mod db;
mod error;
mod status;

use error::AppError;

use app::App;

use crate::db::create_bd;

fn main() -> Result<(), AppError> {
    let conn = create_bd()?;
    ratatui::run(|terminal| App::new(conn).run(terminal))
}
