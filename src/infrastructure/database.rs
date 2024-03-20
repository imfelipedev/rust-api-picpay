use sqlx::postgres::{
    PgPool, 
    PgPoolOptions
};

use std::env;

pub async fn setup() -> Result<PgPool, String> {
    let database_url = match env::var("DATABASE_URL") {
        Ok(path) => path,
        Err(_) => {
            let message_error = "❌ - DATABASE_URL environment variable not set.".to_string();
            return Err(message_error);
        }
    };

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(connection) => connection,
        Err(err) => {
            let message_error = format!("❌ - Failed connect database: {:?}.", err);
            return Err(message_error);
        }
    };

    Ok(pool)
}
