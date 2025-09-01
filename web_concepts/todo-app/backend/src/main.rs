use axum::{routing::{get, post}, Router, Json};
use serde::{Serialize, Deserialize};
use std::env;
use tokio::net::TcpListener;
use sea_orm::{Database, EntityTrait, QuerySelect, ColumnTrait, QueryFilter, Set, TransactionTrait, ActiveModelTrait};
pub mod user;
pub mod todo;
pub mod log;
use utoipa::ToSchema;
use utoipa::path;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


#[derive(OpenApi)]
#[openapi(
    paths(add_todo_transaction, get_todos),  // include your endpoints
    components(schemas(TodoResponse, NewTodo)),
    tags(
        (name = "todo", description = "Todo management endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();

    let db_url = env::var("DATABASE_URL")?;
    let db = Database::connect(&db_url).await?;

    let app = Router::new()
    .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
    .route(
        "/todos",
        get({
            let db = db.clone();
            move || get_todos(db.clone())
        }),
    )
    .route(
        "/add_todo_txn",
        post({
            let db = db.clone();
            move |payload| add_todo_transaction(db.clone(), payload)
        })
    );

    println!(" Backend running on 0.0.0.0:8080");
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}


#[derive(Serialize, ToSchema)]
struct TodoResponse {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Deserialize, ToSchema)]
struct NewTodo {
    title: String,
    user_id: i32,
}


#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List all todos", body = Vec<TodoResponse>)
    )
)]
async fn get_todos(db: sea_orm::DatabaseConnection) -> Json<Vec<TodoResponse>> {
    let todos = todo::Entity::find().all(&db).await.unwrap();
    Json(todos.into_iter().map(|t| TodoResponse {
        id: t.id,
        title: t.title,
        done: t.done,
    }).collect())
}

#[utoipa::path(
    post,
    path = "/add_todo_txn",
    request_body = NewTodo,
    responses(
        (status = 200, description = "Todo added successfully", body = String),
        (status = 500, description = "Transaction failed"),
    )
)]
async fn add_todo_transaction(
    db: sea_orm::DatabaseConnection,
    Json(payload): Json<NewTodo>,
) -> Result<Json<String>, (axum::http::StatusCode, String)> {
    let txn = db.begin().await.map_err(internal_err)?;

    // Insert Todo
    let todo = todo::ActiveModel {
        title: Set(payload.title.clone()),
        done: Set(false),
        user_id: Set(payload.user_id),
        ..Default::default()
    };
    let todo_res = todo.insert(&txn).await.map_err(internal_err)?;

    // Insert Log entry
    let log_entry = log::ActiveModel {
        message: Set(format!("Todo {} created", todo_res.id)),
        todo_id: Set(todo_res.id),
        ..Default::default()
    };

    // Hard check for testing rollback
    let simulate_failure = payload.title == "force_fail";

    if simulate_failure {
        return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Simulated failure — rolled back".to_string(),
        ));
    }

    log_entry.insert(&txn).await.map_err(internal_err)?;

    // Commit only if all succeeded
    txn.commit().await.map_err(internal_err)?;

    Ok(Json(format!("Todo {} inserted with log", todo_res.id)))
}

fn internal_err<E: std::fmt::Display>(
    e: E,
) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
