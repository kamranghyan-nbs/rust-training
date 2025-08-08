pub use self::error::{Error, Result};

use axum::{middleware, routing::get, Router};
use axum::response::{Html, IntoResponse, Response};
use axum::extract::{Path, Query};
use serde::Deserialize;
use crate::model::ModelController;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    
    let mc = ModelController::new().await?;

    // build our application with a single route
    let app = Router::new()
        .merge(routes_hello())
		.merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper));
  
    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!();
    res
}

fn routes_hello() -> Router {
	Router::new()
		.route("/hello", get(handler_hello))
		.route("/hello2/{name}", get(handler_hello2))
}


#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// Handlers
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, <strong>{name}</strong>"))
}