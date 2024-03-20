use std::sync::Arc;

use crate::services::{
    user_service::UserService,
    transaction_service::TransactionService
};

pub struct AppState {
    pub user_service: Arc<UserService>,
    pub transaction_service: Arc<TransactionService>
}
