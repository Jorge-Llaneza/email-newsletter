use email_newsletter::configurations::get_configuration;
use sqlx::PgPool;
use std::net::TcpListener;
mod telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let server = email_newsletter::startup::run(listener, db_pool)?;

    server.await
}
