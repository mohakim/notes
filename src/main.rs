use axum::{Router, extract::State, http::StatusCode, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, fmt};
mod state;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let app_state = AppState::new(&db_url).await?;

    let app = Router::new()
        .route("/health", get(health))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000);

    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("{} listening on http://{addr}", domain::service_name());

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    // Postgres built-in RNG; returns f64 in [0,1)
    let (r,): (f64,) = sqlx::query_as("SELECT random()")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("db error: {e}")))?;
    Ok(format!("{r:.12}")) // keep output compact
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
