use std::env;

use axum::{extract::State, routing::post, Json, Router};
use deadpool::managed::Pool;
use log::info;
use pool::InstancePool;
use types::ClientFightResult;

mod generated;
mod types;
mod fight_instance;
mod pool;

const DB_NAME: &str = "superhero-server";

#[derive(Clone)]
struct AppState {
    // instance: Arc<SpacetimeConnectionInstance>,
    pool: Pool<InstancePool>,
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting client");
    let spacetime_db = env::var("SPACETIME_DB").unwrap_or(DB_NAME.to_owned());
    let spacetime_db_url = env::var("SPACETIME_DB_URL").unwrap_or("http://localhost:3000".to_owned());


    // let instance = SpacetimeConnectionInstance::new(spacetime_db, spacetime_db_url).await;
    let instance_pool = InstancePool {
        db_name: spacetime_db,
        db_url: spacetime_db_url,
    };
    let state = AppState { pool: Pool::builder(instance_pool).max_size(10).build().unwrap() };
    let app = Router::new().route("/random_fight", post(perform_fight))
        .with_state(state);
    info!("Setup complete, service http");
    // run our app with hyper, listening globally on port 9082
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9082").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

#[axum::debug_handler]
async fn perform_fight(State(state): State<AppState>)->Json<ClientFightResult> {
    let instance = state.pool.get().await.unwrap();
    let result = instance.perform_fight().await;
    Json(result)
}
