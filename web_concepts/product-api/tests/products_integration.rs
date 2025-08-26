use reqwest::Client;
use rust_decimal_macros::dec;
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_product_via_mock() {
    // Start a mock HTTP server
    let mock_server = MockServer::start().await;

    // Mock the POST /products endpoint
    let created_product = json!({
        "id": Uuid::new_v4(),
        "name": "Laptop",
        "description": "Gaming laptop",
        "price": dec!(1299.99),
        "quantity": 10,
        "category": "Electronics",
        "created_at": "2025-08-13T10:00:00Z",
        "updated_at": "2025-08-13T10:00:00Z"
    });

    Mock::given(method("POST"))
        .and(path("/products"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&created_product))
        .mount(&mock_server)
        .await;

    // Simulate API client calling our mock server
    let client = Client::new();
    let res = client
        .post(format!("{}/products", &mock_server.uri()))
        .json(&json!({
            "name": "Laptop",
            "description": "Gaming laptop",
            "price": dec!(1299.99),
            "quantity": 10,
            "category": "Electronics"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["name"], "Laptop");
}

#[tokio::test]
async fn test_list_products_via_mock() {
    let mock_server = MockServer::start().await;

    let product_list = json!({
        "products": [
            {
                "id": Uuid::new_v4(),
                "name": "Mouse",
                "description": "Wireless Mouse",
                "price": dec!(25.00),
                "quantity": 50,
                "category": "Electronics",
                "created_at": "2025-08-13T10:00:00Z",
                "updated_at": "2025-08-13T10:00:00Z"
            }
        ],
        "total": 1,
        "page": 1,
        "per_page": 10
    });

    Mock::given(method("GET"))
        .and(path("/products"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&product_list))
        .mount(&mock_server)
        .await;

    let client = Client::new();
    let res = client
        .get(format!("{}/products", &mock_server.uri()))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["total"], 1);
}

#[tokio::test]
async fn test_get_product_by_id() {
    let mock_server = MockServer::start().await;

    let product_id = Uuid::new_v4();
    let product_json = json!({
        "id": product_id,
        "name": "Wireless Mouse",
        "description": "Ergonomic wireless mouse",
        "price": "49.99",
        "quantity": 20,
        "category": "Accessories",
        "created_at": "2025-08-13T12:00:00Z",
        "updated_at": "2025-08-13T12:00:00Z"
    });

    // Mock GET /products/{id}
    Mock::given(method("GET"))
        .and(path(format!("/products/{}", product_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(&product_json))
        .mount(&mock_server)
        .await;

    let resp = reqwest::get(format!("{}/products/{}", &mock_server.uri(), product_id))
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "Wireless Mouse");
}

#[tokio::test]
async fn test_update_product() {
    let mock_server = MockServer::start().await;

    let product_id = Uuid::new_v4();
    let updated_product_json = json!({
        "id": product_id,
        "name": "Updated Mouse",
        "description": "Updated ergonomic mouse",
        "price": "59.99",
        "quantity": 15,
        "category": "Accessories",
        "created_at": "2025-08-13T12:00:00Z",
        "updated_at": "2025-08-13T12:10:00Z"
    });

    // Mock PUT /products/{id}
    Mock::given(method("PUT"))
        .and(path(format!("/products/{}", product_id)))
        .respond_with(ResponseTemplate::new(200).set_body_json(&updated_product_json))
        .mount(&mock_server)
        .await;

    let resp = reqwest::Client::new()
        .put(format!("{}/products/{}", &mock_server.uri(), product_id))
        .json(&json!({
            "name": "Updated Mouse",
            "price": "59.99",
            "quantity": 15
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "Updated Mouse");
}

#[tokio::test]
async fn test_delete_product() {
    let mock_server = MockServer::start().await;

    let product_id = Uuid::new_v4();

    // Mock DELETE /products/{id}
    Mock::given(method("DELETE"))
        .and(path(format!("/products/{}", product_id)))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let resp = reqwest::Client::new()
        .delete(format!("{}/products/{}", &mock_server.uri(), product_id))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 204);
}
