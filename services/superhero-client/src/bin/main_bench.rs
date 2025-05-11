use std::{env, time::{Duration, Instant}};

use async_trait::async_trait;
use clap::Parser;
use deadpool::managed::Pool;
use log::info;

const DB_NAME: &str = "superhero-server";

use rlt::{cli::BenchCli, BenchSuite, IterInfo, IterReport, Status};
use superhero_client::{fight_instance::SpacetimeConnectionInstance, pool::InstancePool};
use tokio::time::sleep;

#[derive(Parser, Clone)]
struct SuperheroBench {
    #[command(flatten)]
    pub bench_opts: BenchCli,
    #[clap(long, default_value = DB_NAME)]
    db_name: String,
    #[clap(long, default_value = "http://localhost:3000")]
    db_url: String,
}

#[async_trait]
impl BenchSuite for SuperheroBench {
    #[doc = " The state for each worker during the benchmark."]
type WorkerState= SpacetimeConnectionInstance;

async fn state(&self, _worker_index: u32) -> anyhow::Result<Self::WorkerState> {
    Ok(SpacetimeConnectionInstance::new(&self.db_name, &self.db_url).await)
}

async fn bench(
    &mut self,
    state: &mut Self::WorkerState,
    _info: &IterInfo,
) -> anyhow::Result<IterReport> {
    let start = Instant::now();
    let res = state.perform_fight().await;
    match res {
        Err(e) => {
            info!("Error: {:?}", e);
            Ok(IterReport { duration: start.elapsed(), items: 0, status: Status::error(0), bytes: 0 })
        }
        Ok(res) => {
            // info!("Fight won by: {}", res.winner_name);
            let duration = start.elapsed();
            Ok(IterReport { duration, items: 1, status: Status::success(0), bytes: 0 })
        }
    }
}
}


#[tokio::main]
pub async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting client");
    // let spacetime_db = env::var("SPACETIME_DB").unwrap_or(DB_NAME.to_owned());
    // let spacetime_db_url =
    //     env::var("SPACETIME_DB_URL").unwrap_or("http://localhost:3000".to_owned());

    
    
    // let instance_pool = InstancePool {
    //     db_name: spacetime_db,
    //     db_url: spacetime_db_url,
    // };
    // let pool: Pool<InstancePool> =  Pool::builder(instance_pool).max_size(10).build().unwrap();

    let bench = SuperheroBench::parse();
    rlt::cli::run(bench.bench_opts, bench).await.unwrap();


    // for i in 0..1000 {
    //     let instance = pool.get().await.unwrap();
    //     let result = instance.perform_fight().await;
    //     // info!("Fight: {} won by: {}", i, result.winner_name);
    
    // }
    info!("Done!")
}
