//! Initial configuration
use crate::{ApiError, Config, get_config};
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};

/// Creates coresponding database with name from [`database_url`](Config::database_url) value.
/// # Errors
/// - Returns [`IoError`](ApiError::IoError) if reading config fails
/// - Returns [`DbError`](ApiError::DbError) if fails in creating new database
pub async fn db() -> Result<(), ApiError> {
	let json: Config = get_config().unwrap_or_default();
	
    let db = Database::connect(json.database_url.clone()).await?;
    match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", json.database_table),
            ))
            .await?;
            let url = format!(
                "{}/{}",
                json.database_url.clone(),
                json.database_table
            );
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", json.database_table),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", json.database_table),
            ))
            .await?;
            let url = format!(
                "{}/{}",
                json.database_url.clone(),
                json.database_table
            );
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(())
}
