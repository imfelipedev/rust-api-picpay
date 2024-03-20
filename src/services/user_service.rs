use axum::http::StatusCode;

use crate::{
    domain::user::{
        user::User, 
        user_query::UserQuery
    },
    repositories::user_repository::UserRepository,
};

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        UserService { user_repository }
    }

    pub async fn get_from_id(&self, id: &i32) -> Result<UserQuery, bool> {
        match self.user_repository.get_from_id(id).await {
            Ok(result) => return Ok(result),
            Err(_) => return Err(false),
        }
    }

    pub async fn update_balance_transaction(&self, payer_id: i32, payee_id: i32, payer_balance: i32, payee_balance: i32) -> Result<bool, bool> {
        match self.user_repository.update_balance_transaction(payer_id, payee_id, payer_balance, payee_balance).await {
            Ok(_) => return Ok(true),
            Err(_) => return Err(false),
        }
    }

    pub async fn register(&self, payload: User) -> (StatusCode, String) {
        if let Ok(_) = self.user_repository.get_from_email(&payload.email).await {
            let response_message = "This email already used.".to_string();
            return (StatusCode::BAD_REQUEST, response_message);
        }

        if let Ok(_) = self.user_repository.get_from_identification(&payload.identification).await {
            let response_message = "This CPF/CNPJ already used.".to_string();
            return (StatusCode::BAD_REQUEST, response_message);
        }

        if let Err(_) = self.user_repository.insert(payload).await {
            let response_message = "Failed create account.".to_string();
            return (StatusCode::INTERNAL_SERVER_ERROR, response_message);
        }

        let response_message = "Account create successfully.".to_string();
        return (StatusCode::CREATED, response_message);
    }
}
