pub mod types;
use log::info;
use spacetimedb::{
    rand::Rng, ProcedureContext, ReducerContext, Table, Timestamp, TxContext,
};
use types::{fight, hero, location, villain, FightResult, Hero, Location, Villain, Winner};

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

#[spacetimedb::reducer]
pub fn clear_fights(ctx: &ReducerContext) -> Result<(), String> {
    let row_count = ctx.db.fight().count();
    for row in ctx.db.fight().iter() {
        ctx.db.fight().delete(row);
    }
    info!("{} fights cleared", row_count);
    Ok(())
}

pub fn all_heroes(ctx: &ReducerContext) -> Vec<Hero> {
    ctx.db.hero().iter().collect()
}

pub fn random_hero(ctx: &TxContext) -> Hero {
    let count = ctx.db.hero().count();
    if count == 0 {
        panic!("Can't pick random hero: No hero found");
    }
    let random_index = ctx.rng().gen_range(1..count + 1);
    ctx.db.hero().space_id().find(random_index).unwrap()
}

pub fn random_villain(ctx: &TxContext) -> Villain {
    let count = ctx.db.villain().count();
    if count == 0 {
        panic!("Can't pick random villain: No villains found");
    }
    let random_index = ctx.rng().gen_range(1..count + 1);
    ctx.db.villain().space_id().find(random_index).unwrap()
}

pub fn random_location(ctx: &TxContext) -> Location {
    let count = ctx.db.location().count();
    if count == 0 {
        panic!("Can't find random location: No location found");
    }

    let random_index = ctx.rng().gen_range(1..count + 1);
    ctx.db.location().space_id().find(random_index).unwrap()
}

pub fn execute_fight(
    hero: Hero,
    villain: Villain,
    location: Location,
    timestamp: Timestamp,
) -> FightResult {
    if hero.level > villain.level {
        FightResult {
            id: 0,
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

#[spacetimedb::procedure]
pub fn execute_random_fight(ctx: &mut ProcedureContext) -> FightResult {
    ctx.with_tx(|ctx| {
        let hero = random_hero(ctx);
        let villain = random_villain(ctx);
        let location = random_location(ctx);
        let result = execute_fight(hero, villain, location, ctx.timestamp);
        ctx.db.fight().insert(result)
    })
}

#[spacetimedb::procedure]
pub fn benchmark_noop(_ctx: &mut ProcedureContext) -> u64 {
    99
}
