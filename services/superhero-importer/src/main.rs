use std::{env, time::Duration};

use generated::{
    DbConnection, Hero, Location, LocationType, Villain, add_hero, add_location, add_villain,
};
use log::{info, warn};
use serde::Deserialize;
use spacetimedb_sdk::DbContext;
use tokio::time::sleep;

pub mod generated;

const DB_NAME: &str = "superhero-server";
const HEROES_JSON: &str = include_str!("data/heroes.json");
const VILLAINS_JSON: &str = include_str!("data/villains.json");
const LOCATIONS_JSON: &str = include_str!("data/locations.json");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CharacterRecord {
    id: i64,
    level: i32,
    name: String,
    other_name: Option<String>,
    picture: String,
    powers: String,
}

#[derive(Debug, Deserialize)]
struct LocationRecord {
    id: i64,
    description: String,
    name: String,
    picture: String,
    #[serde(rename = "type")]
    location_type: LocationTypeRecord,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum LocationTypeRecord {
    City,
    Planet,
    Place,
    Island,
    Country,
    Moon,
}

impl From<LocationTypeRecord> for LocationType {
    fn from(value: LocationTypeRecord) -> Self {
        match value {
            LocationTypeRecord::City => Self::City,
            LocationTypeRecord::Planet => Self::Planet,
            LocationTypeRecord::Place => Self::Place,
            LocationTypeRecord::Island => Self::Island,
            LocationTypeRecord::Country => Self::Country,
            LocationTypeRecord::Moon => Self::Moon,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let spacetime_db = env::var("SPACETIME_DB").unwrap_or(DB_NAME.to_owned());
    let spacetime_db_url =
        env::var("SPACETIME_DB_URL").unwrap_or("http://localhost:3000".to_owned());
    let keep_alive = env::var("IMPORTER_KEEP_ALIVE")
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let db = loop {
        info!("Connecting...");
        match connect_to_client(&spacetime_db, &spacetime_db_url) {
            Ok(db) => break db,
            Err(error) => warn!("Problem connecting: {:?}", error),
        }
        sleep(Duration::from_millis(500)).await;
    };
    let db = Box::leak(Box::new(db));
    tokio::spawn(db.run_async());

    import_heroes(db)
        .await
        .map_err(|_| anyhow::anyhow!("Error importing"))?;
    import_villains(db)
        .await
        .map_err(|_| anyhow::anyhow!("Error importing"))?;
    import_locations(db)
        .await
        .map_err(|_| anyhow::anyhow!("Error importing"))?;
    if keep_alive {
        info!("IMPORTER_KEEP_ALIVE=true; keeping importer connection alive");
        loop {
            sleep(Duration::from_secs(10)).await;
        }
    }

    db.disconnect()
        .map_err(|_| anyhow::anyhow!("Error disconnecting"))
}

async fn import_heroes(db: &DbConnection) -> anyhow::Result<()> {
    let heroes: Vec<CharacterRecord> = load_records(HEROES_JSON, "heroes");
    info!("Loaded {} heroes from JSON", heroes.len());

    for hero in heroes {
        let name = hero.name.clone();
        let id = hero.id;
        db.reducers.add_hero(Hero {
            space_id: 0,
            id,
            level: hero.level,
            name: hero.name,
            other_name: hero.other_name,
            picture: hero.picture,
            powers: hero.powers,
        })?;
        println!("Hero: {} inserted. Id: {}", name, id);
    }
    Ok(())
}

async fn import_villains(db: &DbConnection) -> anyhow::Result<()> {
    let villains: Vec<CharacterRecord> = load_records(VILLAINS_JSON, "villains");
    info!("Loaded {} villains from JSON", villains.len());

    for villain in villains {
        let name = villain.name.clone();
        let id = villain.id;
        db.reducers
            .add_villain(Villain {
                space_id: 0,
                id,
                level: villain.level,
                name: villain.name,
                other_name: villain.other_name,
                picture: villain.picture,
                powers: villain.powers,
            })
            .unwrap();
        println!("Villain: {} inserted. Id: {}", name, id);
    }
    Ok(())
}

async fn import_locations(db: &DbConnection) -> anyhow::Result<()> {
    let locations: Vec<LocationRecord> = load_records(LOCATIONS_JSON, "locations");
    info!("Loaded {} locations from JSON", locations.len());

    for location in locations {
        let name = location.name.clone();
        let id = location.id;
        db.reducers
            .add_location(Location {
                space_id: 0,
                id,
                description: location.description,
                name: location.name,
                picture: location.picture,
                location_type: location.location_type.into(),
            })
            .unwrap();
        println!("Location: {} inserted. Id: {}", name, id);
        sleep(Duration::from_millis(3)).await;
    }
    Ok(())
}

fn load_records<T>(json: &str, name: &str) -> Vec<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_str(json)
        .unwrap_or_else(|error| panic!("Could not parse {name}.json: {error}"))
}

fn connect_to_client(db_name: &str, db_url: &str) -> Result<DbConnection, spacetimedb_sdk::Error> {
    info!("Connecting to spacetimedb. URL: {} DB: {}", db_url, db_name);
    DbConnection::builder()
        .with_uri(db_url)
        .with_database_name(db_name)
        .build()
}
