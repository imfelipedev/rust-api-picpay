use sqlx::{
    Error,
    postgres::{
        PgPool, 
        PgRow
    }
};

use crate::domain::user::{
    user::User, 
    user_query::UserQuery
};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        UserRepository { 
            pool 
        }
    }

    pub async fn get_from_id(&self, id: &i32) -> Result<UserQuery, Error> {
        sqlx::query_as!(UserQuery, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_from_email(&self, email: &String) -> Result<UserQuery, Error> {
        sqlx::query_as!(UserQuery, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_from_identification(&self, identification: &String) -> Result<UserQuery, Error> {
        sqlx::query_as!(UserQuery, "SELECT * FROM users WHERE identification = $1", identification)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn insert(&self, payload: User) -> Result<PgRow, Error> {
        sqlx::query("INSERT INTO users (name, identification, email, password, user_type) VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(payload.name)
            .bind(payload.identification)
            .bind(payload.email)
            .bind(payload.password)
            .bind(payload.user_type)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_balance_transaction(&self, payer_id: i32, payee_id: i32, payer_balance: i32, payee_balance: i32) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("UPDATE users SET balance = $1 WHERE id = $2")
            .bind(payer_balance)
            .bind(payer_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE users SET balance = $1 WHERE id = $2")
            .bind(payee_balance)
            .bind(payee_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await
    }
}
