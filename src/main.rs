use email_newsletter;
use email_newsletter::configurations::get_configuration;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let server = email_newsletter::startup::run(listener)?;
    Ok(())
}
