use crate::domain::authorizer::authorizer::Authorizer;

pub struct AuthorizerService {}

impl AuthorizerService {
    pub fn new() -> Self {
        AuthorizerService {}
    }

    pub async fn verify(&self) -> bool {
        let response = match reqwest::get("https://run.mocky.io/v3/5794d450-d2e2-4412-8131-73d0293ac1cc").await {
            Ok(res) => res,
            Err(_) => return false
        };

        let status = response.status();
        if !status.is_success() {
            return false;
        }

        let body: Authorizer = match response.json().await {
            Ok(json) => json,
            Err(_) => return false
        };

        if body.message != "Autorizado" {
            return false;
        }

        true
    }
}