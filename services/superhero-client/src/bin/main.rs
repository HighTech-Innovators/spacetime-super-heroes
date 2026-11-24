use std::env;

use axum::{
    Json, Router,
    extract::State,
    routing::{delete, get, post},
};
use deadpool::managed::Pool;
use log::info;
use superhero_client::{pool::InstancePool, types::ClientFightResult};

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
    let instance = state.pool.get().await.unwrap();
    let result = instance.perform_fight().await.unwrap();
    let response = Json(result.into());
    response
}

#[axum::debug_handler]
async fn benchmark_noop(State(state): State<AppState>) -> Json<u64> {
    let instance = state.pool.get().await.unwrap();
    let result = instance.noop().await.unwrap();
    let response = Json(result);
    response
}

#[axum::debug_handler]
async fn clear_fights(State(state): State<AppState>) -> Json<u64> {
    let instance = state.pool.get().await.unwrap();
    Json(instance.clear_fights().await.unwrap())
}
