use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use uuid::Uuid;
use std::fmt::Write;

pub async fn issue_newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();

    for message in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", message.content()).unwrap();
    }

    let idempotency_key = Uuid::new_v4();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">

            <head>
              <title>Send Newsletter</title>
              <meta charset="UTF-8">
              <meta name="viewport" content="width=device-width, initial-scale=1">
            </head>

            <body>
              {msg_html}

              <form action="/admin/newsletters" method="post">
                <label>Title</label>
                <br>
                <input type="text" placeholder="Title" name="title">
                <br>
                <label>Plain text content</label>
                <br>
                <textarea placeholder="Plain text content" name="text_content"></textarea>
                <br>
                <label>HTML content</label>
                <br>
                <textarea placeholder="HTML content" name="html_content"></textarea>

                <input hidden type="text" name="idempotency_key" value="{idempotency_key}">

                <br>
                <button type="submit">Publish</button>
              </form>

              <p><a href="/admin/dashboard">&lt;- Back</a></p>
            </body>

            </html>
            "#
        ))
}
