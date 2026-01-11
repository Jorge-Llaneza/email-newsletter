use actix_web::{HttpResponse, web};
use env_logger::Env;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    //TODO if this fn is called more than once it will crash due to the logger init
    #[cfg(debug_assertions)] env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let request_id = Uuid::new_v4();

    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber",
        request_id,
        form.name,
        form.email
    );
    log::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("request_id {} New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("request_id {} Failed to execute query: {:?}",request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
