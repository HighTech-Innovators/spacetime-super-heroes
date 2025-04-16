use spacetimedb::{Identity, SpacetimeType};

#[spacetimedb::table(name = hero, public)]
pub struct Hero {
    #[primary_key]
    #[auto_inc]
    pub space_id: u64,
    #[unique]
    pub id: i64,
    pub level: i32,
    pub name: String,
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}

#[spacetimedb::table(name = villain, public)]
pub struct Villain {
    #[primary_key]
    #[auto_inc]
    pub space_id: u64,
    #[unique]
    pub id: i64,
    pub level: i32,
    pub name: String,
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}

#[derive(SpacetimeType)]
pub enum LocationType {
    CITY,
    PLANET,
    PLACE,
    ISLAND,
    COUNTRY,
    MOON,
}


#[spacetimedb::table(name = event, public)]
pub struct Event {
    pub name: String,
}

#[spacetimedb::table(name = location, public)]
pub struct Location {
    #[primary_key]
    #[auto_inc]
    pub space_id: u64,
    #[unique]
    pub id: i64,
    pub description: String,
    pub name: String,
    pub picture: String,
    pub location_type: LocationType, 
}

#[derive(SpacetimeType)]
pub struct FightLocation {
    pub id: i64,
    pub description: String,
    pub name: String,
    pub picture: String,
    pub location_type: LocationType, 
}

#[spacetimedb::table(name = fight, public)]
pub struct FightResult {
    #[primary_key]
    #[auto_inc]
    #[unique]
    pub id: u64,
    #[index(btree)]
    pub identity: Identity,
    pub request_id: Identity,
    // pub fight_date: Timestamp, // use timestamp data structure?
    pub winner_name: String,
    pub winner_level: i32,
    pub winner_powers: String,
    pub winner_picture: String,
    pub loser_name: String,
    pub loser_level: i32,
    pub loser_powers: String,
    pub loser_picture: String,
    pub winner_team: Winner,
    pub loser_team: Winner,
    pub location: FightLocation,
}

#[derive(Clone, Copy, Debug)]
#[derive(SpacetimeType)]
pub enum Winner {
    Heroes,
    Villains,
}

impl From<Location> for FightLocation {
    fn from(value: Location) -> Self {
        Self { id: value.id, description: value.description, name: value.name, picture: value.picture, location_type: value.location_type }
    }
}