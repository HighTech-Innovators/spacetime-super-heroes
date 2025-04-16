## Spacetime Superheroes
This is a PoC to re-implement [The Quarkus Superheroes application](https://github.com/quarkusio/quarkus-super-heroes) using SpacetimeDB, specifically to investigate energy usage under load.

## The client
At first I want to load the same dataset from all the microservice databases into SpacetimeDB. For now it is three databases (two Postgres, one MariaDB). Each with a very modest amount of data.

I generated some client code, and basically copied the Rust query code from my original. I query all the data from the hero table, loop through the rows, and for each row I call the add_hero reduces:

```rust
#[tokio::main]
async fn main() {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .connect("postgres://superman:superman@localhost:5432/heroes_database")
        .await
        .unwrap();
    println!("Pool created");
    let db = connect_to_client();
    let handle = db.run_threaded();
    for hero in all_heroes(&pool).await {
        let name = hero.name.clone();
        let converted: Hero = hero.into();
        let id = converted.id;
        db.reducers.add_hero(converted).unwrap();
        // sleep(Duration::from_millis(1));        
        println!("Hero: {} inserted. Id: {}", name, id);
    }
}

fn connect_to_client()->DbConnection {
    DbConnection::builder()
        .with_uri("http://localhost:3000")
        .with_module_name(DB_NAME)
        .build()
        .unwrap()
}
```
This will give me a database with only one row, Chewbacca, the first in the database.

Uncommenting the 1-milli sleep fixes it, then there are 100.
(well, 101 in total, I added an extra auto-increment space_id, because I wondered if the original id was the problem. It wasn't.)
