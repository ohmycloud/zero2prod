use actix_web::HttpResponse;
use actix_web::web;

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

// We are prefixing `body` with a `_` to avoid
// a compiler warning about unused arguments
pub async fn publish_newsletter(_body: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
