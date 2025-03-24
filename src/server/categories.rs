#![cfg(feature = "ssr")]
use crate::model::{JdArea, JdCategory};
use crate::server::db::get_connection;
use leptos::server_fn::ServerFnError;
use leptos::server;
use sqlx::Row;

#[server]
pub async fn get_all_categories() -> Result<Vec<JdCategory>, ServerFnError> {
    let mut conn = get_connection().await?;

    // Query categories and join with areas
    let query = r#"
        SELECT
            c.id, c.name, c.description, c.area_id,
            a.id as area_id, a.name as area_name, a.description as area_description
        FROM jd_categories c
        LEFT JOIN jd_areas a ON c.area_id = a.id
        ORDER BY c.id
    "#;

    let rows = sqlx::query(query)
        .fetch_all(&mut conn)
        .await?;

    let mut categories = Vec::new();

    for row in rows {
        let category_id: i64 = row.get("id");
        let area_id: i64 = row.get("area_id");

        categories.push(JdCategory {
            id: category_id,
            name: row.get("name"),
            description: row.get("description"),
            area_id,
            area: Some(JdArea {
                id: row.get("area_id"),
                name: row.get("area_name"),
                description: row.get("area_description"),
            }),
        });
    }

    Ok(categories)
}

#[server]
pub async fn find_category_by_id(id: i64) -> Result<Option<JdCategory>, ServerFnError> {
    let mut conn = get_connection().await?;

    // Query for specific category and join with its area
    let query = r#"
        SELECT
            c.id, c.name, c.description, c.area_id,
            a.id as area_id, a.name as area_name, a.description as area_description
        FROM jd_categories c
        LEFT JOIN jd_areas a ON c.area_id = a.id
        WHERE c.id = ?
        LIMIT 1
    "#;

    let id_i64 = id as i64;

    let result = sqlx::query(query)
        .bind(id_i64)
        .fetch_optional(&mut conn)
        .await?;

    if let Some(row) = result {
        let category_id: i64 = row.get("id");
        let area_id: i64 = row.get("area_id");

        Ok(Some(JdCategory {
            id: category_id,
            name: row.get("name"),
            description: row.get("description"),
            area_id,
            area: Some(JdArea {
                id: row.get("area_id"),
                name: row.get("area_name"),
                description: row.get("area_description"),
            }),
        }))
    } else {
        Ok(None)
    }
}
