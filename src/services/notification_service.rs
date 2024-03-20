use crate::domain::notification::notification::Notification;

pub struct NotificationService {}

impl NotificationService {
    pub fn new() -> Self {
        NotificationService {}
    }

    pub async fn send(&self) -> bool {
        let response = match reqwest::get("https://run.mocky.io/v3/54dc2cf1-3add-45b5-b5a9-6bf7e7f1f4a6").await {
            Ok(res) => res,
            Err(_) => return false
        };

        let status = response.status();
        if !status.is_success() {
            return false;
        }

        let body: Notification = match response.json().await {
            Ok(json) => json,
            Err(_) => return false
        };

        if !body.message {
            return false;
        }

        true
    }
}