use chrono::Utc;
use diesel::Insertable;

use crate::{entity::Date, schema::*};

#[derive(Debug, Insertable)]
#[table_name = "pilot"]
pub struct NewPilot<'a> {
    name: &'a str,
    created_date: Date,
}

impl<'a> NewPilot<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            created_date: Utc::now(),
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "leg"]
pub struct NewLeg<'a> {
    created_by: i32,
    created_date: Date,
    description: Option<&'a str>,
    distance: f64,
    origin: String,
    destination: String,
    additional_waypoints: Option<&'a str>,
}
