use std::time::Duration;

use anyhow::{Result, bail};
use clap::Parser;
use deadpool::managed::Pool;
use log::info;
use superhero_client::pool::InstancePool;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep_until};

const DB_NAME: &str = "superhero-server";
const DEFAULT_CATCH_UP_GRACE_MILLIS: u64 = 1_000;

#[derive(Parser, Debug)]
#[command(about = "Run random fights at a configured request rate")]
struct RateBench {
    #[arg(long, default_value = DB_NAME)]
    db_name: String,
    #[arg(long, default_value = "http://localhost:3000")]
    db_url: String,
    #[arg(long, default_value_t = 20)]
    max_pool_size: usize,
    #[arg(long)]
    rate: u64,
    #[arg(long, default_value_t = 30)]
    duration_secs: u64,
    #[arg(long)]
    iterations: Option<u64>,
    #[arg(long, default_value_t = DEFAULT_CATCH_UP_GRACE_MILLIS)]
    catch_up_grace_millis: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let options = RateBench::parse();
    if options.rate == 0 {
        bail!("--rate must be greater than zero");
    }
    if options.max_pool_size == 0 {
        bail!("--max-pool-size must be greater than zero");
    }
    if options.duration_secs == 0 && options.iterations.is_none() {
        bail!("--duration-secs or --iterations must allow at least one request");
    }

    let target_iterations = match options.iterations {
        Some(iterations) => iterations,
        None => options
            .rate
            .checked_mul(options.duration_secs)
            .ok_or_else(|| anyhow::anyhow!("rate multiplied by duration is too large"))?,
    };
    if target_iterations == 0 {
        bail!("the run must contain at least one iteration");
    }

    let manager = InstancePool {
        db_name: options.db_name,
        db_url: options.db_url,
    };
    let pool: Pool<InstancePool> = Pool::builder(manager)
        .max_size(options.max_pool_size)
        .build()
        .map_err(|error| anyhow::anyhow!("Could not create connection pool: {error}"))?;

    info!(
        "Starting rate run: target_iterations={} rate={} requests/second max_pool_size={}",
        target_iterations, options.rate, options.max_pool_size
    );

    let period = Duration::from_nanos((1_000_000_000 / options.rate).max(1));
    let started = Instant::now();
    let mut next_deadline = started;
    let mut tasks = JoinSet::new();

    for _ in 0..target_iterations {
        sleep_until(next_deadline).await;
        next_deadline += period;

        let pool = pool.clone();
        tasks.spawn(async move {
            let instance = pool
                .get()
                .await
                .map_err(|error| anyhow::anyhow!("Could not get pool instance: {error}"))?;
            instance.perform_fight().await.map(|_| ())
        });
    }

    let mut successful = 0_u64;
    let mut failed = 0_u64;
    while let Some(result) = tasks.join_next().await {
        match result? {
            Ok(()) => successful += 1,
            Err(_) => failed += 1,
        }
    }

    let elapsed = started.elapsed();
    let expected_duration = Duration::from_secs_f64(target_iterations as f64 / options.rate as f64);
    let allowed_duration = expected_duration + Duration::from_millis(options.catch_up_grace_millis);
    let final_pool_size = pool.status().size;
    let achieved_rate = successful as f64 / elapsed.as_secs_f64();

    info!(
        "Rate run complete: iterations={} successful={} failed={} elapsed={:.2}s achieved_rate={:.2} requests/second final_pool_size={}",
        target_iterations,
        successful,
        failed,
        elapsed.as_secs_f64(),
        achieved_rate,
        final_pool_size
    );

    if failed > 0 {
        bail!("rate run failed: {} requests failed", failed);
    }
    if elapsed > allowed_duration {
        bail!(
            "rate run did not keep up: elapsed {:.2}s exceeded expected {:.2}s plus {}ms grace",
            elapsed.as_secs_f64(),
            expected_duration.as_secs_f64(),
            options.catch_up_grace_millis
        );
    }

    Ok(())
}
