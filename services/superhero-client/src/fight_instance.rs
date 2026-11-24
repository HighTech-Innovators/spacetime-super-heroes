use std::sync::atomic::{AtomicU32, Ordering};

use log::info;
use spacetimedb_sdk::error::InternalError;

static NEXT_INSTANCE_NUMBER: AtomicU32 = AtomicU32::new(1);

use crate::generated::{
    DbConnection, FightResult, benchmark_noop, clear_fights, execute_random_fight,
};

pub struct SpacetimeConnectionInstance {
    pub(crate) instance_number: u32,
    pub(crate) connection: &'static DbConnection,
}

impl SpacetimeConnectionInstance {
    pub async fn new(db_name: &str, db_url: &str) -> Self {
        let instance_number = NEXT_INSTANCE_NUMBER.fetch_add(1, Ordering::Relaxed);
        let connection = Self::connect_to_client(&db_name, &db_url, instance_number).unwrap();

        let connection = Box::leak(Box::new(connection));
        let instance = Self {
            instance_number,
            connection,
        };
        tokio::spawn(connection.run_async());
        info!("Instance {} created", instance_number);
        info!("Instance {} job started", instance_number);
        instance
    }

    pub async fn noop(&self) -> anyhow::Result<u64> {
        let (result_sender, result_receiver) =
            tokio::sync::oneshot::channel::<Result<u64, InternalError>>();

        self.connection.procedures.benchmark_noop_then(|_ctx, e| {
            result_sender.send(e).unwrap();
        });
        Ok(result_receiver.into_future().await??)
    }

    pub async fn perform_fight(&self) -> anyhow::Result<FightResult> {
        let (result_sender, result_receiver) =
            tokio::sync::oneshot::channel::<Result<FightResult, InternalError>>();

        self.connection
            .procedures
            .execute_random_fight_then(|_ctx, e| {
                result_sender.send(e).unwrap();
            });
        Ok(result_receiver.into_future().await??)
    }

    pub async fn clear_fights(&self) -> anyhow::Result<u64> {
        let (result_sender, result_receiver) =
            tokio::sync::oneshot::channel::<Result<u64, InternalError>>();
        self.connection
            .procedures
            .clear_fights_then(move |_ctx, result| {
                result_sender.send(result).unwrap();
            });
        let cleared = result_receiver.await??;
        info!(
            "Instance {} cleared {} fights",
            self.instance_number, cleared
        );
        Ok(cleared)
    }

    fn connect_to_client(
        db_name: &str,
        db_url: &str,
        instance_number: u32,
    ) -> Result<DbConnection, spacetimedb_sdk::Error> {
        info!(
            "Instance {} connecting to spacetimedb. URL: {} DB: {}",
            instance_number, db_url, db_name
        );
        DbConnection::builder()
            .with_uri(db_url)
            .with_database_name(db_name)
            .with_confirmed_reads(false)
            .build()
    }
}
