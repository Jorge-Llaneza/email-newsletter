use crate::FormData;
use actix_web::{HttpResponse, web};

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
