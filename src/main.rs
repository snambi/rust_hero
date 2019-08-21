#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json};

mod hero;
use hero::Hero;
use serde_json::json;

#[post("/", data = "<hero>" )]
fn create( hero: Json<Hero>) -> Json<Hero>{
    //println!("Created called : {:?} ", hero);
    hero
}

#[get("/")]
fn read() -> Hero {
    let h = Hero::new(Some(33), String::from_str("John"),
              String::from_str("human"),String::from_str("san jose"),
              34);

    h
}

#[put("/<id>", data="<hero>")]
fn update( id:i32 , hero:Json<Hero> )-> Json<Hero> {
    hero
}

#[delete("/<id>")]
fn delete( id:i32 ) -> Json<Hero>{
    //Json(json!({"status": "ok"}))
    let hero = Hero::new(Some(33), String::from_str("John"),
                         String::from_str("human"),String::from_str("san jose"),
                         34);
    Json( hero )
}


fn main() {
    println!("Starting Rocket");

    rocket::ignite()
        .mount("/hero", routes![create, update, delete] )
        .mount("/heros", routes![read] )
        .launch();

}
