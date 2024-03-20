use serde::Deserialize;

#[derive(Deserialize)]
pub struct Notification {
    pub message: bool
}