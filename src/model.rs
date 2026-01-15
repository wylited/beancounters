use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Transaction {
    pub id: Option<String>, // file:line
    pub date: String,
    pub flag: String,
    pub payee: Option<String>,
    pub narration: Option<String>,
    pub tags: Vec<String>,
    pub postings: Vec<Posting>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Posting {
    pub account: String,
    pub amount: String,
    pub currency: String,
    pub cost: Option<String>,
    pub price: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Account {
    pub name: String,
    pub open_date: String,
    pub currencies: Vec<String>,
    pub close_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CloseAccountRequest {
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
