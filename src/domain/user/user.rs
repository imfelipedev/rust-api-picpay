use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub identification: String,
    pub email: String,
    pub password: String,
    pub user_type: String
}
