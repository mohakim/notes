use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query_as, query_scalar};

use crate::error::*;

#[derive(Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct NoteResponse {
    pub id: i64,
    pub user_id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct DeleteResult {
    pub success: bool,
    pub id: i32,
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateNote>,
) -> AppResult<Json<NoteResponse>> {
    let user_id: i32 = 1;

    let row = query_as!(
        NoteResponse,
        r#"
        INSERT INTO notes (user_id, title, content)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, title, content, created_at
        "#,
        user_id,
        payload.title,
        payload.content
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(row))
}

pub async fn list(
    State(pool): State<PgPool>,
    Query(p): Query<ListParams>,
) -> AppResult<Json<Vec<NoteResponse>>> {
    let limit = p.limit.unwrap_or(50).clamp(1, 200);
    let offset = p.offset.unwrap_or(0).max(0);

    let rows = query_as!(
        NoteResponse,
        r#"
            SELECT id, user_id, title, content, created_at
            FROM notes
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(rows))
}

pub async fn get(State(pool): State<PgPool>, Path(id): Path<i32>) -> AppResult<Json<NoteResponse>> {
    let row = query_as!(
        NoteResponse,
        r#"
            SELECT id, user_id, title, content, created_at
            FROM NOTES
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match row {
        Some(note) => Ok(Json(note)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateNote>,
) -> AppResult<Json<NoteResponse>> {
    let row = query_as!(
        NoteResponse,
        r#"
        UPDATE notes
        SET
            title = COALESCE($1, title),
            content = COALESCE($2, content)
        WHERE id = $3
        RETURNING id, user_id, title, content, created_at
        "#,
        body.title,
        body.content,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match row {
        Some(note) => Ok(Json(note)),
        None => Err(crate::error::AppError::NotFound),
    }
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> AppResult<Json<DeleteResult>> {
    let deleted = query_scalar!(
        r#"
          DELETE FROM notes
          WHERE id = $1
          RETURNING id
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match deleted {
        Some(deleted_id) => Ok(Json(DeleteResult {
            success: true,
            id: deleted_id,
        })),
        None => Err(AppError::NotFound),
    }
}
