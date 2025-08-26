use reqwest::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_register_user() {
    // Start a mock HTTP server
    let mock_server = MockServer::start().await;

    // Fake /register endpoint
    Mock::given(method("POST"))
        .and(path("/register"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "message": "User registered successfully"
        })))
        .mount(&mock_server)
        .await;

    // Send request to mock server
    let client = Client::new();
    let resp = client
        .post(format!("{}/register", &mock_server.uri()))
        .json(&serde_json::json!({
            "username": "testuser",
            "email": "test@example.com",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 201);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["message"], "User registered successfully");
}

#[tokio::test]
async fn test_login_user() {
    let mock_server = MockServer::start().await;

    // Fake /login endpoint
    Mock::given(method("POST"))
        .and(path("/login"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "token": "fake-jwt-token"
        })))
        .mount(&mock_server)
        .await;

    let client = Client::new();
    let resp = client
        .post(format!("{}/login", &mock_server.uri()))
        .json(&serde_json::json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["token"], "fake-jwt-token");
}
