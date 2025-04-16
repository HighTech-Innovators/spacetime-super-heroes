use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::generated::Location;

#[derive(sqlx::Type, Debug)]
pub enum SqlLocationType {
    CITY,
    PLANET,
    PLACE,
    ISLAND,
    COUNTRY,
    MOON,
}

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Default)]
pub struct SqlLocation {
    pub id: i64,
    pub description: String,
    pub name: String,
    pub picture: String,
    // r#type: String, // TODO use enum
}

impl From<SqlLocation> for Location {
    fn from(value: SqlLocation) -> Self {
        // TODO implement LocationType
        Location { space_id: 0, id: value.id, description: value.description, name: value.name, picture: value.picture, location_type: crate::generated::LocationType::City }
    }
}
