use crate::{Error, Result};
use serde::Deserialize;
use axum::routing::post;
use serde_json::{json, Value};
use axum::{Json, Router};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // TODO : Implement db/auth logic
    if payload.username !="demo" || payload.password != "welcome" {
        return Err(Error::LoginFail)
    }

    // TODO : Set cookies

    // Create the success body
    let body = Json(json!({
        "result" : {
            "success" : true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}