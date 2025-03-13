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
