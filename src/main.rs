mod api;
mod beancount;
mod model;
mod state;

use axum::{
    response::Html,
    routing::{get, put},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::list_transactions,
        api::add_transaction,
        api::update_transaction,
        api::delete_transaction,
        api::clear_transaction,
        api::unclear_transaction,
        api::list_accounts,
        api::add_account,
        api::update_account,
        api::delete_account,
        api::close_account,
        api::verify_ledger
    ),
    components(
        schemas(model::Transaction, model::Posting, model::Account, model::VerifyResult, model::CloseAccountRequest)
    ),
    tags(
        (name = "beancounters", description = "Beancount API")
    )
)]
struct ApiDoc;

// TODO fix this
async fn scalar_ui() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>API Reference</title>
  <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
  <style>
    body { margin: 0; }
    #scalar-reference { width: 100vw; height: 100vh; }
  </style>
</head>
<body>
  <script>
    const configuration = {
      spec: {
        url: '/api-docs/openapi.json'
      },
      // Optional: customize theme, hide download button, etc.
      // branding: { logo: false },
      // meta: { hideDownloadButton: true }
    }
    document.getElementById('scalar-reference').clientConfig = configuration
  </script>
  <scalar-api-reference id="scalar-reference" />
</body>
</html>
    "#)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "beancounters=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = state::AppState::new("data".to_string())?;

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/references", get(scalar_ui))
        .route("/transactions", get(api::list_transactions).post(api::add_transaction))
        .route("/transactions/{id}", put(api::update_transaction).delete(api::delete_transaction))
        .route("/transactions/{id}/clear", axum::routing::post(api::clear_transaction))
        .route("/transactions/{id}/unclear", axum::routing::post(api::unclear_transaction))
        .route("/accounts", get(api::list_accounts).post(api::add_account))
        .route("/accounts/{name}", put(api::update_account).delete(api::delete_account))
        .route("/accounts/{name}/close", axum::routing::post(api::close_account))
        .route("/verify", get(api::verify_ledger))
        .with_state(Arc::new(app_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
