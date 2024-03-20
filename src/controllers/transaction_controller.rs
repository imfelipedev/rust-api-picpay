use axum::{
    body::Body,
    response::Response,
    extract::{
        Json, 
        State
    }
};

use std::sync::Arc;

use crate::domain::{
    appstate::appstate::AppState, 
    transaction::transaction::Transaction
};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Transaction>,
) -> Response {
    let (status, message) = state.transaction_service.create(payload).await;
    let response_body = Body::from(message);
    Response::builder()
        .status(status)
        .body(response_body)
        .unwrap()
}
