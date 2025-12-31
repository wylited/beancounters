use axum::{extract::{State, Path}, Json, http::StatusCode};
use std::sync::Arc;
use crate::state::AppState;
use crate::model::{Transaction, Account, VerifyResult};
use crate::beancount;

#[utoipa::path(
    get,
    path = "/transactions",
    responses(
        (status = 200, description = "List all transactions", body = Vec<Transaction>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_transactions(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Transaction>>, StatusCode> {
    match beancount::list_transactions(&state.data_dir) {
        Ok(txs) => Ok(Json(txs)),
        Err(e) => {
            tracing::error!("Failed to list transactions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    post,
    path = "/transactions",
    request_body = Transaction,
    responses(
        (status = 201, description = "Transaction created"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_transaction(State(state): State<Arc<AppState>>, Json(payload): Json<Transaction>) -> StatusCode {
    let _lock = state.write_lock.lock().unwrap();
    match beancount::add_transaction(&state.data_dir, payload) {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            tracing::error!("Failed to add transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[utoipa::path(
    put,
    path = "/transactions/{id}",
    params(
        ("id" = String, Path, description = "Transaction ID")
    ),
    request_body = Transaction,
    responses(
        (status = 200, description = "Transaction updated"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_transaction(State(state): State<Arc<AppState>>, Path(id): Path<String>, Json(payload): Json<Transaction>) -> StatusCode {
    let _lock = state.write_lock.lock().unwrap();
    match beancount::update_transaction(&id, payload) {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Failed to update transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[utoipa::path(
    delete,
    path = "/transactions/{id}",
    params(
        ("id" = String, Path, description = "Transaction ID")
    ),
    responses(
        (status = 200, description = "Transaction deleted"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_transaction(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> StatusCode {
    let _lock = state.write_lock.lock().unwrap();
    match beancount::delete_transaction(&id) {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Failed to delete transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[utoipa::path(
    get,
    path = "/accounts",
    responses(
        (status = 200, description = "List all accounts", body = Vec<Account>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_accounts(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Account>>, StatusCode> {
    match beancount::list_accounts(&state.data_dir) {
        Ok(accounts) => Ok(Json(accounts)),
        Err(e) => {
            tracing::error!("Failed to list accounts: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    post,
    path = "/accounts",
    request_body = Account,
    responses(
        (status = 201, description = "Account created"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_account(State(state): State<Arc<AppState>>, Json(payload): Json<Account>) -> StatusCode {
    let _lock = state.write_lock.lock().unwrap();
    match beancount::add_account(&state.data_dir, payload) {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            tracing::error!("Failed to add account: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[utoipa::path(
    put,
    path = "/accounts/{name}",
    params(
        ("name" = String, Path, description = "Account name")
    ),
    request_body = Account,
    responses(
        (status = 200, description = "Account updated"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_account(State(state): State<Arc<AppState>>, Path(name): Path<String>, Json(payload): Json<Account>) -> StatusCode {
    let _lock = state.write_lock.lock().unwrap();
    match beancount::update_account(&state.data_dir, &name, payload) {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Failed to update account: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[utoipa::path(
    delete,
    path = "/accounts/{name}",
    params(
        ("name" = String, Path, description = "Account name")
    ),
    responses(
        (status = 200, description = "Account deleted"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_account(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> StatusCode {
    let _lock = state.write_lock.lock().unwrap();
    match beancount::delete_account(&state.data_dir, &name) {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Failed to delete account: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[utoipa::path(
    get,
    path = "/verify",
    responses(
        (status = 200, description = "Verify ledger", body = VerifyResult),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn verify_ledger(State(state): State<Arc<AppState>>) -> Result<Json<VerifyResult>, StatusCode> {
    match beancount::verify(&state.data_dir) {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            tracing::error!("Failed to verify ledger: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
