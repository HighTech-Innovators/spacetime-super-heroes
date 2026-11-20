use std::env;

use axum::{
    Json, Router,
    extract::State,
    routing::{delete, get, post},
};
use deadpool::managed::Pool;
use log::info;
use superhero_client::{pool::InstancePool, types::ClientFightResult};
use tokio::time::Instant;

const DB_NAME: &str = "superhero-server";

#[derive(Clone)]
struct AppState {
    pool: Pool<InstancePool>,
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting client");
    let spacetime_db = env::var("SPACETIME_DB").unwrap_or(DB_NAME.to_owned());
    let spacetime_db_url =
        env::var("SPACETIME_DB_URL").unwrap_or("http://localhost:3000".to_owned());

    let instance_pool = InstancePool {
        db_name: spacetime_db,
        db_url: spacetime_db_url,
    };
    let state = AppState {
        pool: Pool::builder(instance_pool).max_size(20).build().unwrap(),
    };
    let app = Router::new()
        .route("/random_fight", post(perform_fight))
        .route("/fights", delete(clear_fights))
        .route("/ping", get(benchmark_ping))
        .route("/noop", get(benchmark_noop))
        .with_state(state);
    info!("Setup complete, service http");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn benchmark_ping() -> &'static str {
    "{}"
}

#[axum::debug_handler]
async fn perform_fight(State(state): State<AppState>) -> Json<ClientFightResult> {
    let start = Instant::now();

    // Measure Deadpool connection acquisition.
    let instance = state.pool.get().await.unwrap();
    let pool_time = start.elapsed();

    // Measure SpacetimeDB procedure execution + response.
    let result = instance.perform_fight().await.unwrap();
    let procedure_time = start.elapsed() - pool_time;

    // Measure total handler time, including conversion.
    let response = Json(result.into());
    let total_time = start.elapsed();

    info!(
        "Fight request completed: {} {} {}",
        pool_time.as_secs_f64() * 1000.0,
        procedure_time.as_secs_f64() * 1000.0,
        total_time.as_secs_f64() * 1000.0,
    );

    response
}

#[axum::debug_handler]
async fn benchmark_noop(State(state): State<AppState>) -> Json<u64> {
    let start = Instant::now();

    // Measure Deadpool connection acquisition.
    let instance = state.pool.get().await.unwrap();
    let pool_time = start.elapsed();

    // Measure SpacetimeDB procedure execution + response.
    let result = instance.noop().await.unwrap();
    let procedure_time = start.elapsed() - pool_time;

    // Measure total handler time, including conversion.
    let response = Json(result);
    let total_time = start.elapsed();

    info!(
        "Fight request completed: {} {} {}",
        pool_time.as_secs_f64() * 1000.0,
        procedure_time.as_secs_f64() * 1000.0,
        total_time.as_secs_f64() * 1000.0,
    );

    response
}

#[axum::debug_handler]
async fn clear_fights(State(state): State<AppState>) -> String {
    let instance = state.pool.get().await.unwrap();
    instance.clear_fights().await.unwrap(); // TODO handle properly
    "Fights cleared".to_owned()
}
