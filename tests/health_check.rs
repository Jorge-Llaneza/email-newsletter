use std::net::SocketAddr;
use tokio;

#[tokio::test]
async fn health_check_works() -> std::io::Result<()> {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let response = client.get(format!("http://{}/health_check", app_address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let valid_bodies = vec!["name=le%20guin&email=ursula_le_guin%40gmail.com",
                            "name=le%20guin&email=ursula_le_guin%40g.co",
                            "name=leguin&email=ursula_le_guin%40gmail.com"];
    for body in valid_bodies {
        let response = client.post(format!("http://{}/subscriptions", app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await.expect("Failed to execute request.");
        assert!(response.status().is_success());
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let invalid_bodies = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (body, error_message) in invalid_bodies {
        let response = client.post(format!("http://{}/subscription", app_address))
            .body(body)
            .send()
            .await.expect("Failed to execute request.");

        assert!(response.status().is_client_error(), "{}", error_message);
    }
}

fn spawn_app() -> SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let app_address = listener.local_addr().unwrap();

    let server = email_newsletter::startup::run(listener).expect("App could't run");
    let _ = tokio::spawn(server);
    app_address
}
