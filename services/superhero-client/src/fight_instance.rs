use log::info;
use rand::{RngCore, SeedableRng};
use rand_isaac::IsaacRng;
use spacetimedb_sdk::{DbContext, Identity, Table};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::{generated::{execute_random_fight, DbConnection, FightResult, FightTableAccess, HeroTableAccess, LocationTableAccess, VillainTableAccess}, types::ClientFightResult};

// #[derive(Clone)]
pub struct SpacetimeConnectionInstance {
    // db_name: String,
    // db_url: String,
    pub(crate) connection: DbConnection,
    pub(crate) identity: Option<Identity>,
    receiver: Receiver<FightResult>,

}

// static IDENTITY: LazyLock<Mutex<Option<Identity>>> = LazyLock::new(|| Mutex::new(None));

// impl Manager for SpacetimeConnectionManager {
//     type Type = DbConnection;

//     type Error = spacetimedb_sdk::Error;

//     fn create(&self) -> impl Future<Output = Result<Self::Type, Self::Error>> + Send {
//         async {
//             let db = Self::connect_to_client(&self.db_name, &self.db_url).unwrap();
//             db.frame_tick()
//             db.reducers.
//             let (sender,receiver) = tokio::sync::broadcast::channel::<FightResult>(100);
//             let identity = IDENTITY.lock().unwrap().un;
//             Self::run_job(&db, identity, sender);
//             // self.connections.push(db);
//             Ok(db)
//         }
//     }

//     fn recycle(
//         &self,
//         obj: &mut Self::Type,
//         _metrics: &deadpool::managed::Metrics,
//     ) -> impl Future<Output = deadpool::managed::RecycleResult<Self::Error>> + Send {
//         async {
//             obj.disconnect()
//                 .map_err(|e| deadpool::managed::RecycleError::Backend(e))
//         }
//     }
// }

impl SpacetimeConnectionInstance {
    pub async fn new(db_name: String, db_url: String)->Self {
        let (sender,receiver) = tokio::sync::broadcast::channel::<FightResult>(100);
        let (connection,identity_receiver) = loop {
            let (identity_sender, identity_receiver) = tokio::sync::oneshot::channel::<Identity>();
            match Self::connect_to_client(&db_name, &db_url, identity_sender) {
                Ok(connection) => break (connection,identity_receiver),
                Err(_) => {},
            }
        };
        let mut instance = Self {
            connection: connection,
            identity: None,
            receiver: receiver,
        };
        info!("Instance created");
        instance.run_job(identity_receiver,sender).await;
        info!("Job started");
        instance
    }

    pub fn frame_tick(&self)->Result<(), spacetimedb_sdk::Error> {
        self.connection.frame_tick()
    }

 
pub async fn perform_fight(&self)->ClientFightResult {
    let mut rng = IsaacRng::from_os_rng();
    let mut id_block = [0_u8;32];
    rng.fill_bytes(&mut id_block);
    let random_id = Identity::from_byte_array(id_block);

    let mut receiver = self.receiver.resubscribe();
    self.connection.reducers.execute_random_fight(self.identity.unwrap(), random_id).unwrap();
    let fight_result = loop {
        let result = receiver.recv().await.unwrap();
        if random_id == result.request_id {
            break result.into();
        }
    };
    fight_result
}

    // pub async fn 
    async fn run_job(&mut self, identity_receiver: tokio::sync::oneshot::Receiver<Identity>, sender: Sender<FightResult>) {
        let identity = identity_receiver.await.unwrap();
        self.identity = Some(identity);
        self.connection.subscription_builder()
                .on_applied(move |e| {
                    info!("Number of fights: {}",e.db.fight().count()); 
                    info!("Number of heroes: {}",e.db.hero().count()); 
                    info!("Number of villains: {}",e.db.villain().count()); 
                    info!("Number of locations: {}",e.db.location().count()); 
                })
                // .subscribe_to_all_tables();
                .subscribe(
                    // Actually, I only actually need to listen for fight, as I'm only defering random heroes, villains and locations to the reducer
                    [
                    format!("SELECT * FROM fight WHERE identity = 0x{}",identity),
                    "SELECT * FROM hero".to_owned(),            
                    "SELECT * FROM villain".to_owned(),            
                    "SELECT * FROM location".to_owned(),            
                ]);
    
        self.connection.db.fight().on_insert(move |_ctx,fight_result| {
            // info!("Fight competed. Winner: {}",fight_result.winner_name);
            sender.send(fight_result.clone()).unwrap();
        });
    }
    
    fn connect_to_client(db_name: &str, db_url: &str, identity_sender: tokio::sync::oneshot::Sender<Identity>)->Result<DbConnection,spacetimedb_sdk::Error> {
        info!("Connecting to spacetimedb. URL: {} DB: {}",db_url, db_name);
        DbConnection::builder()
            .with_uri(db_url)
            .with_module_name(db_name)
            .on_connect(move |_db,identity, _token| {
                identity_sender.send(identity).unwrap();
                // IDENTITY.lock().unwrap().replace(identity);
                // info!("Connected, identity: {}",identity);
            })
            .build()
    }
}
