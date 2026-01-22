use email_newsletter::configurations::get_configuration;
use email_newsletter::telemetry;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        telemetry::get_subscriber("Email-Newsletter".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let server = email_newsletter::startup::run(listener, db_pool)?;

    server.await
}
