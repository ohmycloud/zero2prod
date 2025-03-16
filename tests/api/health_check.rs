use wiremock::http;

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {}

#[tokio::test]
async fn test_received_request() {
    use http::Method;
    use wiremock::MockServer;

    // Arrange
    let mock_server = MockServer::start().await;

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let address = format!("{}/subscriptions", &mock_server.uri());
    surf::post(&address)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .await
        .unwrap();

    // Assert
    let received_requests = mock_server.received_requests().await.unwrap();
    assert_eq!(received_requests.len(), 1);

    let received_request = &received_requests[0];
    assert_eq!(received_request.method, Method::POST);
    assert_eq!(received_request.url.path(), "/subscriptions");
    assert!(!received_request.body.is_empty());
}
