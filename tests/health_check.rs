use std::net::SocketAddr;
use tokio;
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() -> std::io::Result<()> {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client.get(format!("http://{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    Ok(())
}

fn spawn_app() -> SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();

    let server = email_newsletter::run(listener).expect("App could't run");
    let _ = tokio::spawn(server);
    address

}
