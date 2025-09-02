use anyhow::{Context, Result};
use axum::extract::FromRef;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app: &AppState) -> Self {
        app.db.clone()
    }
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self> {
        let opts: PgConnectOptions = database_url.parse()?;
        let db = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect_with(opts.statement_cache_capacity(0))
            .await
            .with_context(|| format!("failed to connect to Postgres at {}", database_url))?;

        sqlx::query("SELECT 1")
            .execute(&db)
            .await
            .context("database ping failed (SELECT 1)")?;

        Ok(Self { db })
    }
}
