use std::time::Instant;

use async_trait::async_trait;
use clap::Parser;
use log::info;
use rlt::{BenchSuite, IterInfo, IterReport, Status, cli::BenchCli};
use superhero_client::fight_instance::SpacetimeConnectionInstance;

const DB_NAME: &str = "superhero-server";

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
    type WorkerState = SpacetimeConnectionInstance;

    async fn state(&self, worker_index: u32) -> anyhow::Result<Self::WorkerState> {
        info!("Creating connection for benchmark worker {}", worker_index);
        Ok(SpacetimeConnectionInstance::new(&self.db_name, &self.db_url).await)
    }

    async fn bench(
        &mut self,
        state: &mut Self::WorkerState,
        _info: &IterInfo,
    ) -> anyhow::Result<IterReport> {
        let start = Instant::now();
        let result = state.perform_fight().await;

        match result {
            Err(error) => {
                info!("Fight failed: {:?}", error);
                Ok(IterReport {
                    duration: start.elapsed(),
                    items: 0,
                    status: Status::error(0),
                    bytes: 0,
                })
            }
            Ok(_) => Ok(IterReport {
                duration: start.elapsed(),
                items: 1,
                status: Status::success(0),
                bytes: 0,
            }),
        }
    }
}

#[tokio::main]
pub async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting client benchmark");
    let bench = SuperheroBench::parse();
    rlt::cli::run(bench.bench_opts, bench).await.unwrap();
    info!("Done!");
}
