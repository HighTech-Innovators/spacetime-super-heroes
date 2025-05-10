use serde::Serialize;
use spacetimedb_sdk::Identity;

use crate::generated::{FightLocation, FightResult, LocationType, Winner};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientFightResult {
    pub id: u64,
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
    pub winner_team: ClientWinner,
    pub loser_team: ClientWinner,
    pub location: ClientFightLocation,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientWinner {
    Heroes,
    Villains,
}

impl From<Winner> for ClientWinner {
    fn from(value: Winner) -> Self {
        match value {
            Winner::Heroes => ClientWinner::Heroes,
            Winner::Villains => ClientWinner::Villains,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientFightLocation {
    pub id: i64,
    pub description: String,
    pub name: String,
    pub picture: String,
    pub location_type: ClientLocationType,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientLocationType {
    City,
    Planet,
    Place,
    Island,
    Country,
    Moon,
}

impl From<LocationType> for ClientLocationType {
    fn from(value: LocationType) -> Self {
        match value {
            LocationType::City => ClientLocationType::City,
            LocationType::Planet => ClientLocationType::Planet,
            LocationType::Place => ClientLocationType::Place,
            LocationType::Island => ClientLocationType::Island,
            LocationType::Country => ClientLocationType::Country,
            LocationType::Moon => ClientLocationType::Moon,
        }
    }
}
impl From<FightLocation> for ClientFightLocation {
    fn from(value: FightLocation) -> Self {
        Self {
            id: value.id,
            description: value.description,
            name: value.name,
            picture: value.picture,
            location_type: value.location_type.into(),
        }
    }
}

impl From<FightResult> for ClientFightResult {
    fn from(value: FightResult) -> Self {
        Self {
            id: value.id,
            identity: value.identity,
            request_id: value.request_id,
            winner_name: value.winner_name,
            winner_level: value.winner_level,
            winner_powers: value.winner_powers,
            winner_picture: value.winner_picture,
            loser_name: value.loser_name,
            loser_level: value.loser_level,
            loser_powers: value.loser_powers,
            loser_picture: value.loser_picture,
            winner_team: value.winner_team.into(),
            loser_team: value.loser_team.into(),
            location: value.location.into(),
        }
    }
}
