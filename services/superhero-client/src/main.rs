use std::{env, sync::{Arc, LazyLock, Mutex}, time::Duration};

use axum::{extract::State, routing::post, Json, Router};
use generated::{execute_random_fight, DbConnection, FightResult, FightTableAccess, HeroTableAccess};
use log::{info, warn};
use rand::{RngCore, SeedableRng};
use rand_isaac::IsaacRng;
use spacetimedb_sdk::{DbContext, Identity, Table};
use tokio::{sync::broadcast::{Receiver, Sender}, time::sleep};
use types::ClientFightResult;

mod generated;
mod types;

const DB_NAME: &str = "superhero-server";
static IDENTITY: LazyLock<Mutex<Option<Identity>>> = LazyLock::new(|| Mutex::new(None));


#[derive(Clone)]
struct AppState {
    identity: Identity,
    db: &'static DbConnection,
    receiver: Arc<Receiver<FightResult>>,
}

#[tokio::main]
async fn main() {
    let spacetime_db = env::var("SPACETIME_DB").unwrap_or(DB_NAME.to_owned());
    let spacetime_db_url = env::var("SPACETIME_DB_URL").unwrap_or("http://localhost:3000".to_owned());

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

        let db = loop {
            info!("Starting>>>>>>>>");
            let db = connect_to_client(&spacetime_db, &spacetime_db_url);
            match db {
                Ok(db) => break db,
                Err(e) => warn!("Problem connecting: {:?}",e),
            }
            sleep(Duration::from_millis(500)).await;
        };
          
    let db = Box::leak(Box::new(db));

    info!("About to run");
    tokio::spawn(db.run_async());
    // db.run_async().await.unwrap();

    let identity = loop {
        if let Some(id) = *IDENTITY.lock().unwrap() {
            break id
        }
        info!("Waiting for identity");
        sleep(Duration::from_millis(500)).await;
    };
    let (sender,receiver) = tokio::sync::broadcast::channel::<FightResult>(100);
    run_job(&db, identity, sender);
    
    let mut count = 0;
    let server_id = identity.clone();
    let app = Router::new().route("/random_fight", post(perform_fight))
        .with_state(AppState { identity: identity.clone(), db, receiver: Arc::new(receiver)});

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // loop {

    //     let mut rng = IsaacRng::from_os_rng();
    //     let mut id_block = [0_u8;32];
    //     rng.fill_bytes(&mut id_block);
    //     let random_id = Identity::from_byte_array(id_block);
    
    //     // db.reducers.add_event(format!("Event#{}",count)).unwrap();
    //     db.reducers.execute_random_fight(identity, random_id).unwrap();        
    //     // sleep(Duration::from_secs(1));
    //     sleep(Duration::from_secs(1)).await;
    //     info!("Number of fights: {}",db.db.fight().count());
    //     info!("Number of heroes: {}",db.db.hero().count());
    //     info!("Sleeping...");
    //     count+=1;
    // }
}

#[axum::debug_handler]
async fn perform_fight(State(state): State<AppState>)->Json<ClientFightResult> {
    let mut rng = IsaacRng::from_os_rng();
    let mut id_block = [0_u8;32];
    rng.fill_bytes(&mut id_block);
    let random_id = Identity::from_byte_array(id_block);
    let mut receiver = state.receiver.resubscribe();

    state.db.reducers.execute_random_fight(state.identity, random_id.clone()).unwrap();
    let fight_result = loop {
        let result = receiver.recv().await.unwrap();
        if random_id == result.request_id {
            break result.into();
        }
    };
    Json(fight_result)
}

fn run_job(db: &DbConnection, identity: Identity, sender: Sender<FightResult>) {
    db.subscription_builder()
            .on_applied(move |e| {
                info!("Number of fights: {}",e.db.fight().count()); 
                info!("Number of heroes: {}",e.db.hero().count()); 
            })
            // .subscribe_to_all_tables();
            .subscribe(
                [
                format!("SELECT * FROM fight WHERE identity = 0x{}",identity),
                "SELECT * FROM hero".to_owned(),            
                "SELECT * FROM villain".to_owned(),            
            ]);

    db.db.fight().on_insert(move |_ctx,fight_result| {
        // info!("Fight competed. Winner: {}",fight_result.winner_name);
        sender.send(fight_result.clone()).unwrap();
    });
}

fn connect_to_client(db_name: &str, db_url: &str)->Result<DbConnection,spacetimedb_sdk::Error> {
    info!("Connecting to spacetimedb. URL: {} DB: {}",db_url, db_name);
    DbConnection::builder()
        .with_uri(db_url)
        .with_module_name(db_name)
        .on_connect(move |_db,identity, _token| {
            IDENTITY.lock().unwrap().replace(identity);
            info!("Connected, identity: {}",identity);
        })
        .build()
}