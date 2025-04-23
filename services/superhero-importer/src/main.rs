use std::{env, time::Duration};

use generated::{add_hero, add_location, add_villain, DbConnection, Hero, Location, Villain};
use log::{info, warn};
use sql::{heroes::SqlHero, location::SqlLocation, villains::SqlVillain};
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, query_as};
use tokio::time::sleep;

pub mod generated;
pub mod sql;

const DB_NAME: &str = "superhero-server";


#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let heroes_db_url = env::var("HEROES_DB_URL").unwrap_or("postgres://superman:superman@localhost:5432/heroes_database".to_owned());
    let villains_db_url = env::var("VILLAINS_DB_URL").unwrap_or("postgres://superman:superman@localhost:5433/villains_database".to_owned());
    let locations_db_url = env::var("LOCATIONS_DB_URL").unwrap_or("mysql://locations:locations@localhost/locations_database".to_owned());
    let spacetime_db = env::var("SPACETIME_DB").unwrap_or(DB_NAME.to_owned());
    let spacetime_db_url = env::var("SPACETIME_DB_URL").unwrap_or("http://localhost:3000".to_owned());

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
    tokio::spawn(db.run_async());

    import_heroes(db, &heroes_db_url).await;
    import_villains(db, &villains_db_url).await;
    import_locations(db, &locations_db_url).await;

    // give the outbound requests 
    sleep(Duration::from_secs(2)).await;

}

// async fn perform_import(State(state): State<AppState>)->String {
//     import_heroes(&state.db, &state.heroes_db_url).await;
//     import_villains(&state.db, &state.villains_db_url).await;
//     import_locations(&state.db, &state.locations_db_url).await;

//     "ok".to_owned()
// }

async fn import_heroes(db: &DbConnection, url: &str) {
    let pool = loop {
        match PgPoolOptions::new()
        .connect(url)
        .await {
            Ok(pool) => break pool,
            Err(_) => {},
        }
        tokio::time::sleep(Duration::from_millis(100)).await
    };
    query_as::<_, SqlHero>("select * from Hero")
        .fetch_all(&pool)
        .await
        .into_iter()
        .flat_map(|heroes| heroes.into_iter())
        .for_each(|hero| {
            let name = hero.name.clone();
            let converted: Hero = hero.into();
            let id = converted.id;
            db.reducers.add_hero(converted).unwrap();
            // sleep(Duration::from_millis(3));        
            println!("Hero: {} inserted. Id: {}", name, id);
        });
}

async fn import_villains(db: &DbConnection, url: &str) {
    let pool = loop {
        match PgPoolOptions::new()
        .connect(url)
        .await {
            Ok(pool) => break pool,
            Err(_) => {},
        }
        tokio::time::sleep(Duration::from_millis(100)).await
    };
    query_as::<_, SqlVillain>("select * from Villain")
        .fetch_all(&pool)
        .await
        .into_iter()
        .flat_map(|villains| villains.into_iter())
        .for_each(|villain| {
            let name = villain.name.clone();
            let converted: Villain = villain.into();
            let id = converted.id;
            db.reducers.add_villain(converted).unwrap();
            std::thread::sleep(Duration::from_millis(3));        
            println!("Villain: {} inserted. Id: {}", name, id);
        });
}

async fn import_locations(db: &DbConnection, url: &str) {
    let pool = loop {
        match MySqlPoolOptions::new()
        .max_connections(30)
        .connect(url)
        .await {
            Ok(pool) => break pool,
            Err(_) => {
                warn!("Location database not up yet")
            },
        }
        tokio::time::sleep(Duration::from_millis(100)).await
    };
    query_as::<_, SqlLocation>("select * from locations")
        .fetch_all(&pool)
        .await
        .into_iter()
        .flat_map(|locations| locations.into_iter())
        .for_each(|location| {
            let name = location.name.clone();
            let converted: Location = location.into();
            let id = converted.id;
            db.reducers.add_location(converted).unwrap();
            std::thread::sleep(Duration::from_millis(3));        
            println!("Location: {} inserted. Id: {}", name, id);
        });
}


fn connect_to_client(db_name: &str, db_url: &str)->Result<DbConnection,spacetimedb_sdk::Error> {
    info!("Connecting to spacetimedb. URL: {} DB: {}",db_url, db_name);
    DbConnection::builder()
        .with_uri(db_url)
        .with_module_name(db_name)
        .build()
}

