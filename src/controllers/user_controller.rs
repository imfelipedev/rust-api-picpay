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
    user::user::User
};

pub async fn register(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<User>
) -> Response {
    let (status, message) = state.user_service.register(payload).await;
    let response_body = Body::from(message);
    Response::builder()
        .status(status)
        .body(response_body)
        .unwrap()
}
