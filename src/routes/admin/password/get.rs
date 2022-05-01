use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

use crate::session_state::TypedSession;
use crate::utils::{error500, see_other};

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(error500)?.is_none() {
        return Ok(see_other("/login"));
    };

    let mut msg_html = String::new();

    for message in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", message.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">

            <head>
              <title>Change Password</title>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1">
            </head>

            <body>
              {msg_html}

              <form action="/admin/password">
                <label>Current Password
                  <input type="password" placeholder="Enter current password" name="current_password">
                </label>
                <br>
                <label>New Password
                  <input type="password" placeholder="Enter new password" name="new_password">
                </label>
                <br>
                <label>Confirm new Password
                  <input type="password" placeholder="Confirm your password" name="new_password_check">
                </label>

                <button type="submit">Change password</button>
              </form>

              <p><a href="/admin/dashboard">&lt;- Back</a></p>
            </body>

            </html>
            "#
        )))
}