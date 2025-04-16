## Spacetime Superheroes
This is a PoC to re-implement [The Quarkus Superheroes application](https://github.com/quarkusio/quarkus-super-heroes) using SpacetimeDB, specifically to investigate energy usage under load.

The original is a Java application, that can run both in JIT and AOT mode. I already created a Rust port and that was pretty straightforward (efficient too).

My 'research question' is: If we move the code into the database and process data 'from the inside', is that more efficient than dragging the data around all the time, through PG Wire protocol or HTTP.

## Approach
Based on the chat app from the documentation, the first thing I tried was take my Rust types, and slap on some macros. The Hero struct is the only one I've gotten around to:

```rust
#[spacetimedb::table(name = hero, public)]
pub struct Hero {
    #[primary_key]
    #[auto_inc]
    pub space_id: i64,
    pub id: i64,
    pub level: i32,
    pub name: String,
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}
```
I added a reducer to be able to insert data:
(I left my desparate log statements in, as timing seemed to influence this problem, I didn't want to clean it up.)
```rust
#[spacetimedb::reducer]
pub fn add_hero(ctx: &ReducerContext, hero: Hero)->Result<(), String> {
    debug!("About to insert.....!");
    info!("About to insert {} - {} - {}",&hero.name, &hero.id, hero.space_id);
    let inserted = ctx.db.hero().insert(hero);
    info!("Hero added: {} id: {} space_id: {}",inserted.name, inserted.id, inserted.space_id);
    Ok(())
}
```

I generated the client code:
```
spacetime generate --lang rust --out-dir ../superhero-client/src/generated
```

(more in the client repository)