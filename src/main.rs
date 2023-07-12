#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashSet;
use std::sync::Mutex;
use rand::Rng;

use serde::Deserialize;

struct IdGenerator {
    set: Mutex<HashSet<u64>>,
}

#[derive(Deserialize, Debug)]
struct User {
    active: bool,
    id: Option<u64>,
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct Team {
    team_id: Option<u64>,
    team_name: String,
    users: Option<Vec<User>>,
    active: bool,
}

fn generate_id(ids: &State<IdGenerator>) -> Result<u64, &'static str> {
    let mut set = ids.set.lock().map_err(|_| "Failed to acquire lock")?;
    let mut rng = rand::thread_rng();

    for _ in 0..10 {  // Limited attempts to find a unique ID
        let generated_id = rng.gen();
        if set.insert(generated_id) {
            return Ok(generated_id);
        }
    }
    Err("Failed to generate a unique ID")
}

#[post("/user", format="json", data="<user>")]
fn create_user(user: Json<User>, ids: &State<IdGenerator>) {
    let user_id = generate_id(ids).unwrap();
    println!("{:?}", user_id);
    let created_user = User { id: Some(user_id), ..user.into_inner() };

    println!("{:?}", created_user)
}

#[post("/team", format="json", data="<team>")]
fn create_team(team: Json<Team>) {

}

#[put("/team/<team_id>/<user_id>")]
fn add_user_to_team(team_id: u64, user_id: u64) {

}

#[delete("/user/<user_id>")]
fn delete_user(user_id: u64) {

}

#[get("/user/<user_id>")]
fn get_user(user_id: u64) {
    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(IdGenerator { set: Mutex::new(HashSet::new()) })
        .mount("/",
        routes![create_user, create_team, add_user_to_team, delete_user, get_user])
}
