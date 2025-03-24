#![cfg(feature = "ssr")]
use crate::model::JdArea;
use crate::server::db::get_connection;
use leptos::server_fn::ServerFnError;
use leptos::server;
use sqlx::Row;

#[server]
pub async fn get_all_areas() -> Result<Vec<JdArea>, ServerFnError> {
    let mut conn = get_connection().await?;

    // Simple query for all areas
    let query = "SELECT id, name, description FROM jd_areas ORDER BY id";

    let rows = sqlx::query(query)
        .fetch_all(&mut conn)
        .await?;

    let areas = rows.into_iter()
        .map(|row| JdArea {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
        })
        .collect();

    Ok(areas)
}