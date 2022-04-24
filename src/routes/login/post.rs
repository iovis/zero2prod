use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse};
use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

pub async fn login(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
