use dashline_data::{entity::{Pilot, Date}, service::PilotService, model::NewPilot};
use serde::{Deserialize, Serialize};
use rocket::{serde::json::Json, State, fairing::AdHoc, routes};

#[macro_use] extern crate rocket;

struct Config {
    database_url: String,
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let config = Config {
        database_url: std::env::var("DATABASE_URL").expect("database url not provided"),
    };

    rocket::build().attach(managed_config_stage(config))
}

fn managed_config_stage(config: Config) -> AdHoc {
    AdHoc::on_ignite("Managed Configuration", |rocket| async {
        rocket
            .mount("/", routes![get, post])
            .manage(config)
    })
}

#[get("/pilot")]
fn get(config: &State<Config>) -> Result<Json<Vec<ResponsePilot>>, ()> {
    let service = PilotService::new(&config.database_url).unwrap();
    let pilots = service.list().unwrap();
    let pilots = pilots.into_iter().map(Pilot::into).collect();
    Ok(Json(pilots))
}

#[derive(Debug, Deserialize)]
struct PostPilot {
    name: String,
}

impl PostPilot {
    fn new_pilot(&self) -> NewPilot {
        NewPilot::new(&self.name)
    }
}

#[derive(Debug, Serialize)]
struct ResponsePilot {
    id: i32,
    name: String,
    created_date: Date,
}

impl From<Pilot> for ResponsePilot {
    fn from(Pilot { id, name, created_date }: Pilot) -> Self {
        Self { id, name, created_date }
    }
}

#[post("/pilot", format = "application/json", data = "<pilot>")]
fn post(pilot: Json<PostPilot>, config: &State<Config>) -> Result<Json<ResponsePilot>, ()> {
    let service = PilotService::new(&config.database_url).unwrap();
    let pilot = service.insert(&pilot.new_pilot()).unwrap().into();
    Ok(Json(pilot))
}
