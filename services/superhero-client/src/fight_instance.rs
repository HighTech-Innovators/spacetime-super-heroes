use log::info;
use rand::{RngCore, SeedableRng};
use rand_isaac::IsaacRng;
use spacetimedb_sdk::{DbContext, Identity, Table};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::{
    generated::{
        DbConnection, FightResult, FightTableAccess, HeroTableAccess, LocationTableAccess,
        VillainTableAccess, execute_random_fight,
    },
    types::ClientFightResult,
};

pub struct SpacetimeConnectionInstance {
    pub(crate) connection: &'static DbConnection,
    pub(crate) identity: Option<Identity>,
    receiver: Receiver<FightResult>,
}

impl SpacetimeConnectionInstance {
    pub async fn new(db_name: String, db_url: String) -> Self {
        let (sender, receiver) = tokio::sync::broadcast::channel::<FightResult>(100);
        let (connection, identity_receiver) = loop {
            let (identity_sender, identity_receiver) = tokio::sync::oneshot::channel::<Identity>();
            match Self::connect_to_client(&db_name, &db_url, identity_sender) {
                Ok(connection) => break (connection, identity_receiver),
                Err(_) => {}
            }
        };
        let connection = Box::leak(Box::new(connection));
        let mut instance = Self {
            connection: connection,
            identity: None,
            receiver: receiver,
        };
        tokio::spawn(connection.run_async());
        info!("Instance created");
        instance.run_job(identity_receiver, sender).await;
        info!("Job started");
        instance
    }

    pub async fn perform_fight(&self) -> ClientFightResult {
        let mut rng = IsaacRng::from_os_rng();
        let mut id_block = [0_u8; 32];
        rng.fill_bytes(&mut id_block);
        let random_id = Identity::from_byte_array(id_block);

        let mut receiver = self.receiver.resubscribe();
        self.connection
            .reducers
            .execute_random_fight(self.identity.unwrap(), random_id)
            .unwrap();
        let fight_result = loop {
            let result = receiver.recv().await.unwrap();
            if random_id == result.request_id {
                break result.into();
            }
        };
        fight_result
    }

    // pub async fn
    async fn run_job(
        &mut self,
        identity_receiver: tokio::sync::oneshot::Receiver<Identity>,
        sender: Sender<FightResult>,
    ) {
        let identity = identity_receiver.await.unwrap();
        self.identity = Some(identity);
        self.connection
            .subscription_builder()
            .on_applied(move |e| {
                info!("Number of fights: {}", e.db.fight().count());
                info!("Number of heroes: {}", e.db.hero().count());
                info!("Number of villains: {}", e.db.villain().count());
                info!("Number of locations: {}", e.db.location().count());
            })
            // .subscribe_to_all_tables();
            .subscribe(
                // Actually, I only actually need to listen for fight, as I'm only defering random heroes, villains and locations to the reducer
                [
                    format!("SELECT * FROM fight WHERE identity = 0x{}", identity),
                    "SELECT * FROM hero".to_owned(),
                    "SELECT * FROM villain".to_owned(),
                    "SELECT * FROM location".to_owned(),
                ],
            );

        self.connection
            .db
            .fight()
            .on_insert(move |_ctx, fight_result| {
                sender.send(fight_result.clone()).unwrap();
            });
    }

    fn connect_to_client(
        db_name: &str,
        db_url: &str,
        identity_sender: tokio::sync::oneshot::Sender<Identity>,
    ) -> Result<DbConnection, spacetimedb_sdk::Error> {
        info!("Connecting to spacetimedb. URL: {} DB: {}", db_url, db_name);
        DbConnection::builder()
            .with_uri(db_url)
            .with_module_name(db_name)
            .on_connect(move |_db, identity, _token| {
                identity_sender.send(identity).unwrap();
            })
            .build()
    }
}
