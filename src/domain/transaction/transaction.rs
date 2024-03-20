use serde::Deserialize;

#[derive(Deserialize)]
pub struct Transaction {
    pub value: i32,
    pub payer: i32,
    pub payee: i32
}
