use email_newsletter::configurations::{get_configuration, DatabaseSettings};
use sqlx::{Connection, PgConnection, PgPool};
use std::net::{SocketAddr, TcpListener};
use reqwest::get;
use tokio;
use email_newsletter::get_db_connection;
use email_newsletter::startup::run;

#[tokio::test]
async fn health_check_works() -> std::io::Result<()> {
    let connection = get_db_connection().await;
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.url_address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let connection = get_db_connection().await;
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let valid_bodies = vec!["name=le%20guin&email=ursula_le_guin%40gmail.com"];
    for body in valid_bodies {
        let response = client
            .post(format!("{}/subscriptions", app.url_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(response.status().is_success());
    }
    let mut connection = get_db_connection().await;
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let invalid_bodies = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (body, error_message) in invalid_bodies {
        let response = client
            .post(format!("{}/subscription", app.url_address))
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_client_error(), "{}", error_message);
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await.expect("Failed to connect to PostgresSQL database.");

    let server = run(listener, connection_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        url_address: address,
        db_pool: connection_pool,
    }
}

struct TestApp {
    url_address: String,
    db_pool: PgPool,
}
