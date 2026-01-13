use email_newsletter::configurations::get_configuration;
use email_newsletter::telemetry;
use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("Email-Newsletter".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let server = email_newsletter::startup::run(listener, db_pool)?;

    server.await
}
