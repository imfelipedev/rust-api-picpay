use axum::{
    Router,
    routing::post,
};

use sqlx::PgPool;

use std::sync::Arc;

use crate::{
    controllers::{
        transaction_controller, 
        user_controller
    }, 
    domain::appstate::appstate::AppState, 
    repositories::user_repository::UserRepository, 
    services::{
        authorizer_service::AuthorizerService, 
        notification_service::NotificationService,
        transaction_service::TransactionService, 
        user_service::UserService
    }
};

pub fn setup(pool: PgPool) -> Router {
    let user_repository = UserRepository::new(pool);

    let user_service = Arc::new(
        UserService::new(user_repository)
    );

    let authorizer_service = AuthorizerService::new();

    let notification_service = NotificationService::new();

    let transaction_service = Arc::new(
        TransactionService::new(
            user_service.clone(),
            authorizer_service,
            notification_service
        )
    );

    let state = Arc::new(AppState {
        user_service,
        transaction_service,
    });

    Router::new()
        .route("/api/v1/user/register", post(user_controller::register))
        .route("/api/v1/transaction/create",post(transaction_controller::create))
        .with_state(state)
}
