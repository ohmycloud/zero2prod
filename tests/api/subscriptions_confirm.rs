use anyhow::Ok;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    // Arrange
    let app = spawn_app().await;

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

    app.post_subscriptions(body.into()).await;

    let received_requests = &app.email_server.received_requests().await.unwrap();
    assert_eq!(received_requests.len(), 1);
    let email_request = &received_requests[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    // Act
    let response = reqwest::get(confirmation_links.html).await?;
    // Assert
    assert_eq!(response.status().as_u16(), 200);
    Ok(())
}
