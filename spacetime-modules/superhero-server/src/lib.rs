pub mod types;
use log::info;
use spacetimedb::{
    rand::Rng, Identity, ReducerContext, Table, Timestamp
};
use types::{event, fight, hero, location, villain, Event, FightResult, Hero, Location, Villain, Winner};

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    info!("Environment init!!")
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {
    info!("Client connected!")
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(_ctx: &ReducerContext) {
    info!("Disconnected client!!");
}

#[spacetimedb::reducer]
pub fn add_hero(ctx: &ReducerContext, hero: Hero) -> Result<(), String> {
    let inserted = ctx.db.hero().insert(hero);
    info!("Hero added: {} id: {}", inserted.name, inserted.id);
    Ok(())
}

#[spacetimedb::reducer]
pub fn add_villain(ctx: &ReducerContext, villain: Villain) -> Result<(), String> {
    let inserted = ctx.db.villain().insert(villain);
    info!("Hero added: {} id: {} ", inserted.name, inserted.id);
    Ok(())
}

#[spacetimedb::reducer]
pub fn add_location(ctx: &ReducerContext, location: Location) -> Result<(), String> {
    let inserted = ctx.db.location().insert(location);
    info!("Hero added: {} id: {}", inserted.name, inserted.id);
    Ok(())
}



pub fn all_heroes(ctx: &ReducerContext) -> Vec<Hero> {
    ctx.db.hero().iter().collect()
}

pub fn random_hero(ctx: &ReducerContext) -> Hero {
    let count = ctx.db.hero().count();
    if count == 0 {
        panic!("Can't pick random hero: No hero found");
    }
    let random_index = ctx.rng().gen_range(1..count+1);
    ctx.db.hero().space_id().find(random_index).unwrap()
}

pub fn random_villain(ctx: &ReducerContext) -> Villain {
    let count = ctx.db.villain().count();
    if count == 0 {
        panic!("Can't pick random villain: No villains found");
    }
    let random_index = ctx.rng().gen_range(1..count+1);
    ctx.db.villain().space_id().find(random_index).unwrap()
}

pub fn random_location(ctx: &ReducerContext) -> Location {
    let count = ctx.db.location().count();
    if count == 0 {
        panic!("Can't find random location: No location found");
    }
    
    let random_index = ctx.rng().gen_range(1..count+1);
    ctx.db.location().space_id().find(random_index).unwrap()
}

pub fn execute_fight(identity: Identity, request_id: Identity, hero: Hero, villain: Villain, location: Location, timestamp: Timestamp) -> FightResult {
    info!("Hero level: {} Villain level: {}",hero.level,villain.level);
    if hero.level > villain.level {
        FightResult {
            id: 0,
            identity,
            request_id,
            fight_date: timestamp,
            winner_name: hero.name,
            winner_level: hero.level,
            winner_powers: hero.powers,
            winner_picture: hero.picture,
            loser_name: villain.name,
            loser_level: villain.level,
            loser_powers: villain.powers,
            loser_picture: villain.picture,
            winner_team: Winner::Heroes,
            loser_team: Winner::Villains,
            location: location.into(),
        }
    } else {
        FightResult {
            id: 0,
            identity,
            request_id,
            fight_date: timestamp,
            winner_name: villain.name,
            winner_level: villain.level,
            winner_powers: villain.powers,
            winner_picture: villain.picture,
            loser_name: hero.name,
            loser_level: hero.level,
            loser_powers: hero.powers,
            loser_picture: hero.picture,
            winner_team: Winner::Villains,
            loser_team: Winner::Heroes,
            location: location.into(),
        }
    }
}

#[spacetimedb::reducer]
pub fn execute_random_fight(ctx: &ReducerContext, identity: Identity, request_id: Identity) -> Result<(), String> {
    info!("Execute random fight");
    let hero = random_hero(ctx);

    info!("Random hero: {}", hero.name);
    let villain = random_villain(ctx);
    info!("Random villain: {}", villain.name);
    let location = random_location(ctx);
    info!("Random location: {}", location.name);
    let result = execute_fight(identity, request_id, hero, villain, location, ctx.timestamp);
    let inserted = ctx.db.fight().insert(result);
    info!("Fight result: {:?}", inserted.winner_name);
    let fight_count = ctx.db.fight().count();
    info!("# of fights: {}", fight_count);
    Ok(())
}

#[spacetimedb::reducer]
pub fn add_event(ctx: &ReducerContext, name: String) -> Result<(), String> {
    info!("Inserting event: {}",name);
    ctx.db.event().insert(Event { name });
    Ok(())
}


#[spacetimedb::reducer]
pub fn add_fight_result(ctx: &ReducerContext, fight_result: FightResult) -> Result<(), String> {
    info!(
        "About to insert fight with winner: {}",
        &fight_result.winner_name
    );
    let inserted = ctx.db.fight().insert(fight_result);
    info!("Fight added with winning team: {:?}", inserted.winner_team);
    Ok(())
}
