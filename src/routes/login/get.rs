use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let _error = query.0.error;

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}
