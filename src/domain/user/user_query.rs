use sqlx::FromRow;

#[derive(FromRow)]
pub struct UserQuery {
    pub id: i32,
    pub name: String,
    pub identification: String,
    pub email: String,
    pub password: String,
    pub balance: i32,
    pub user_type: String
}