use chrono::{DateTime, Duration, Utc};
use diesel::Queryable;

pub type Date = DateTime<Utc>;

#[derive(Clone, Debug, Queryable)]
pub struct Pilot {
    pub id: i32,
    pub name: String,
    pub created_date: Date,
}

#[derive(Clone, Debug, Queryable)]
pub struct Leg {
    pub id: i32,
    pub created_by: i32,
    pub created_date: Date,
    pub description: Option<String>,
    pub distance: f64,
    pub origin: String,
    pub destination: String,
    pub additional_waypoints: Option<String>,
}

#[derive(Clone, Debug, Queryable)]
pub struct Flight {
    pub id: i32,
    pub pilot_id: i32,
    pub leg_id: i32,
    pub created_date: Date,
    pub completed_date: String,
    pub remarks: Option<String>,
    pub aircraft: String,
    pub duration: Duration,
}
