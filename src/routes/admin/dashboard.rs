use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::authentication::UserId;
use crate::utils::error500;

pub async fn admin_dashboard(
    user_id: web::ReqData<UserId>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = get_username(*user_id.into_inner(), &pool)
        .await
        .map_err(error500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width" />
                <title>Admin Dashboard</title>
            </head>
            <body>
                <p>Welcome {username}!</p>
                <p>Available actions:</p>
                <ol>
                    <li><a href="/admin/password">Change password</a></li>
                    <li><a href="/admin/newsletters">Send a newsletter issue</a></li>
                    <li>
                        <form name="logoutForm" action="/admin/logout" method="post">
                            <input type="submit" value="Logout">
                        </form>
                    </li>
                </ol>
            </body>
            </html>
            "#
        )))
}

pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;

    Ok(row.username)
}
