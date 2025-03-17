use anyhow::Ok;
use reqwest::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    // Arrange
    let app = spawn_app().await;
    let confirmation_token = uuid::Uuid::new_v4().to_string();

    // Act
    let response = reqwest::Client::new()
        .get(&format!("{}/subscriptions/confirm", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() -> anyhow::Result<()> {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // let mock_server = MockServer::start().await;
    // let address = format!("{}/subscriptions", &mock_server.uri());
    // let response = surf::post(address)
    //     .header("Content-Type", "application/x-www-form-urlencoded")
    //     .body(body)
    //     .await
    //     .unwrap();
    //
    reqwest::Client::new()
        .post(format!("{}/subscriptions", &app.email_server.uri()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    app.post_subscriptions(body.into()).await;

    let received_requests = &app.email_server.received_requests().await.unwrap();
    assert_eq!(received_requests.len(), 1);
    let email_request = &received_requests[0];
    let body: serde_json::Value = email_request.body_json().unwrap();

    // Extract the link from one of the request fields.
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    let raw_confirmation_link = &get_link(&body["HtmlBody"].as_str().unwrap());
    let mut confirmation_link = Url::parse(raw_confirmation_link)?;

    // Let's make sure we don't call random APIs on the web
    assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
    // Let's rewrite the URL to include the port
    confirmation_link.set_port(Some(app.port)).unwrap();

    // Act
    let response = reqwest::get(confirmation_link).await?;
    // Assert
    assert_eq!(response.status().as_u16(), 200);
    Ok(())
}
