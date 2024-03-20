use axum::http::StatusCode;

use std::sync::Arc;

use crate::{
    domain::transaction::transaction::Transaction,
    services::{
        user_service::UserService, 
        authorizer_service::AuthorizerService,
        notification_service::NotificationService
    }
};

pub struct TransactionService {
    user_service: Arc<UserService>,
    authorizer_service: AuthorizerService,
    notification_service: NotificationService
}

impl TransactionService {
    pub fn new(
        user_service: Arc<UserService>, 
        authorizer_service: AuthorizerService,
        notification_service: NotificationService
    ) -> Self {
        TransactionService {
            user_service,
            authorizer_service,
            notification_service
        }
    }

    pub async fn create(&self, payload: Transaction) -> (StatusCode, String) {
        if payload.payer == payload.payee {
            let response_message = "The Payer must be different from the Payee.".to_string();
            return (StatusCode::BAD_REQUEST, response_message);
        }

        let mut user = match self.user_service.get_from_id(&payload.payer).await {
            Ok(user) => user,
            Err(_) => {
                let response_message = "Payer account not found.".to_string();
                return (StatusCode::BAD_REQUEST, response_message);
            }
        };

        let mut receiver = match self.user_service.get_from_id(&payload.payee).await {
            Ok(receiver) => receiver,
            Err(_) => {
                let response_message = "Payee account not found.".to_string();
                return (StatusCode::BAD_REQUEST, response_message);
            }
        };

        let type_not_allowed = "Merchant".to_string();
        if user.user_type == type_not_allowed {
            let response_message = "Payer account is not allowed to send money.".to_string();
            return (StatusCode::UNAUTHORIZED, response_message);
        }

        if user.balance < payload.value || user.balance <= 0 {
            let response_message = "Payer account not have sufficient balance.".to_string();
            return (StatusCode::UNAUTHORIZED, response_message);
        }

        if !self.authorizer_service.verify().await {
            let response_message = "External authorization failed.".to_string();
            return (StatusCode::UNAUTHORIZED, response_message);
        }

        if !self.notification_service.send().await {
            let response_message = "External notification failed.".to_string();
            return (StatusCode::INTERNAL_SERVER_ERROR, response_message);
        }

        user.balance -= payload.value;

        receiver.balance += payload.value;

        if let Err(_) = self.user_service.update_balance_transaction(payload.payer, payload.payee, user.balance, receiver.balance).await {
            let response_message = "Failed to update data.".to_string();
            return (StatusCode::INTERNAL_SERVER_ERROR, response_message);
        }

        let response_message = "transaction completed successfully.".to_string();
        return (StatusCode::ACCEPTED, response_message);
    }
}
