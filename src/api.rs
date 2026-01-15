use axum::{extract::{State, Path}, Json, http::StatusCode, response::IntoResponse};
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
pub async fn list_transactions(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let data_dir = state.data_dir.clone();
    tokio::task::spawn_blocking(move || beancount::list_transactions(&data_dir))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to list transactions: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
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
pub async fn add_transaction(State(state): State<Arc<AppState>>, Json(payload): Json<Transaction>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let _lock = state.write_lock.lock().unwrap();
        beancount::add_transaction(&state.data_dir, payload)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
    .map(|_| StatusCode::CREATED)
    .map_err(|e| {
        tracing::error!("Failed to add transaction: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
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
pub async fn update_transaction(State(state): State<Arc<AppState>>, Path(id): Path<String>, Json(payload): Json<Transaction>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let _lock = state.write_lock.lock().unwrap();
        beancount::update_transaction(&state.data_dir, &id, payload)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
    .map(|_| StatusCode::OK)
    .map_err(|e| {
        tracing::error!("Failed to update transaction: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
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
pub async fn delete_transaction(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let _lock = state.write_lock.lock().unwrap();
        beancount::delete_transaction(&id)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
    .map(|_| StatusCode::OK)
    .map_err(|e| {
        tracing::error!("Failed to delete transaction: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
}

#[utoipa::path(
    get,
    path = "/accounts",
    responses(
        (status = 200, description = "List all accounts", body = Vec<Account>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_accounts(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Account>>, (StatusCode, String)> {
    let data_dir = state.data_dir.clone();
    tokio::task::spawn_blocking(move || beancount::list_accounts(&data_dir))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to list accounts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
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
pub async fn add_account(State(state): State<Arc<AppState>>, Json(payload): Json<Account>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let _lock = state.write_lock.lock().unwrap();
        beancount::add_account(&state.data_dir, payload)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
    .map(|_| StatusCode::CREATED)
    .map_err(|e| {
        tracing::error!("Failed to add account: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
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
pub async fn update_account(State(state): State<Arc<AppState>>, Path(name): Path<String>, Json(payload): Json<Account>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let _lock = state.write_lock.lock().unwrap();
        beancount::update_account(&state.data_dir, &name, payload)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
    .map(|_| StatusCode::OK)
    .map_err(|e| {
        tracing::error!("Failed to update account: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
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
pub async fn delete_account(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let _lock = state.write_lock.lock().unwrap();
        beancount::delete_account(&state.data_dir, &name)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
    .map(|_| StatusCode::OK)
    .map_err(|e| {
        tracing::error!("Failed to delete account: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
}

#[utoipa::path(
    get,
    path = "/verify",
    responses(
        (status = 200, description = "Verify ledger", body = VerifyResult),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn verify_ledger(State(state): State<Arc<AppState>>) -> Result<Json<VerifyResult>, (StatusCode, String)> {
    let data_dir = state.data_dir.clone();
    tokio::task::spawn_blocking(move || beancount::verify(&data_dir))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Task join error: {}", e)))?
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to verify ledger: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
}
