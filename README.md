# Beancounters API

Hey there! 👋 This is a simple, local API server for managing your Beancount ledger. It's built with Rust, Axum, and a few other cool tools.

## What it does

It lets you programmatically:
*   **Manage Transactions**: Add, list, update, and delete transactions.
*   **Manage Accounts**: Open and close accounts (well, mostly open/list/delete for now).
*   **Verify**: Check your ledger for errors using the parser.

It works directly with your `.bean` files in the `data/` directory.

## Getting Started

1.  **Run it**:
    ```bash
    cargo run
    ```
    It'll start listening on `http://localhost:3000`.

2.  **Explore the API**:
    *   **Scalar UI**: Check out the beautiful API reference at [http://localhost:3000/references](http://localhost:3000/references).
    *   **Swagger UI**: Prefer the classic look? Go to [http://localhost:3000/docs](http://localhost:3000/docs).
    *   **OpenAPI Spec**: Need the raw JSON? It's at [http://localhost:3000/api-docs/openapi.json](http://localhost:3000/api-docs/openapi.json).

## Data Structure

The server expects a `data/` directory with:
*   `main.bean`: The entry point.
*   `accounts.bean`: Your account definitions.
*   `YYYY-MM.bean`: Monthly transaction files (created automatically).

Enjoy counting those beans! 🫘
