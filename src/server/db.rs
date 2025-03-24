#![cfg(feature = "ssr")]
use leptos::server_fn::ServerFnError;
use sqlx::{Connection, SqliteConnection};

pub async fn get_connection() -> Result<SqliteConnection, ServerFnError> {
    Ok(SqliteConnection::connect("sqlite:db.sqlite").await?)
}