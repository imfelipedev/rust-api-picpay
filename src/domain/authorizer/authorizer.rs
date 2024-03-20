use serde::Deserialize;

#[derive(Deserialize)]
pub struct Authorizer {
    pub message: String
}