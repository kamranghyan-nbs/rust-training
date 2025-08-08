use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/ticket/{id}", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
	State(mc): State<ModelController>,
	Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {

	let ticket = mc.create_ticket(ticket_fc).await?;

	Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}
