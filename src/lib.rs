use secrecy::ExposeSecret;
use crate::configurations::get_configuration;
use sqlx::{Connection, PgConnection};

pub mod configurations;
pub mod routes;
pub mod startup;
pub mod telemetry;
///gets a connection to the development Postgres db
pub async fn get_db_connection() -> PgConnection {
    let configuration = get_configuration().expect("Failed to read configuration");
    PgConnection::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres")
}
