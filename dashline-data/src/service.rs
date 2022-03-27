use diesel::prelude::*;

use crate::{
    entity::{Leg, Pilot},
    model::{NewLeg, NewPilot},
    schema::*,
};

pub struct PilotService {
    connection: PgConnection,
}

impl PilotService {
    pub fn new(database_url: &str) -> crate::Result<Self> {
        Ok(Self {
            connection: PgConnection::establish(database_url)?,
        })
    }

    pub fn insert(&self, new_pilot: &NewPilot) -> crate::Result<Pilot> {
        Ok(diesel::insert_into(pilot::table)
            .values(new_pilot)
            .get_result(&self.connection)?)
    }

    pub fn list(&self) -> crate::Result<Vec<Pilot>> {
        Ok(pilot::dsl::pilot.load(&self.connection)?)
    }
}

pub struct LegService {
    connection: PgConnection,
}

impl LegService {
    pub fn new(database_url: &str) -> crate::Result<Self> {
        Ok(Self {
            connection: PgConnection::establish(database_url)?,
        })
    }

    pub fn insert(&self, new_plan: &NewLeg) -> crate::Result<Leg> {
        Ok(diesel::insert_into(leg::table)
            .values(new_plan)
            .get_result(&self.connection)?)
    }
}
