use std::env;

use axum::{Json, Router, extract::{Path, State}, http::Response, routing::{delete, post}};
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
        pool: Pool::builder(instance_pool).max_size(10).build().unwrap(),
    };
    let app = Router::new()
        .route("/random_fight", post(perform_fight))
        .route("/random_fights/{count}", post(perform_fights))
        .route("/fights", delete(clear_fights))
        .with_state(state);
    info!("Setup complete, service http");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn perform_fight(State(state): State<AppState>) -> Json<ClientFightResult> {
    let instance = state.pool.get().await.unwrap();
    Json(instance.perform_fight().await.unwrap())
}

#[axum::debug_handler]
async fn perform_fights(Path(count): Path<u32>, State(state): State<AppState>) -> Json<Vec<ClientFightResult>> {
    info!("Received fight request for {} fights", count);
    let instance = state.pool.get().await.unwrap();
    Json(instance.perform_fights(count).await.unwrap())
}

#[axum::debug_handler]
async fn clear_fights(State(state): State<AppState>)->String {
    let instance = state.pool.get().await.unwrap();
    instance.clear_fights().await.unwrap(); // TODO handle properly
    "Fights cleared".to_owned()
}