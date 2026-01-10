use sqlx::{Connection, PgConnection};
use crate::configurations::get_configuration;

pub mod configurations;
pub mod routes;
pub mod startup;

pub async fn get_db_connection() -> PgConnection {
    let configuration = get_configuration().expect("Failed to read configuration");
    PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres")
}