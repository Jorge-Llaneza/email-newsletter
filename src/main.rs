use std::net::TcpListener;
use email_newsletter;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let server = email_newsletter::startup::run(listener)?;
        Ok(())
}