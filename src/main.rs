use email_newsletter;
use email_newsletter::configurations::get_configuration;
use env_logger::Env;
use sqlx::{Connection, PgPool};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let server = email_newsletter::startup::run(listener, db_pool)?;

    server.await;
    Ok(())
}
