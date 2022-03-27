use std::fmt::Display;

#[macro_use]
extern crate diesel;

pub mod entity;
pub mod model;
pub mod service;

// Diesel module
mod schema;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Connection(diesel::ConnectionError),
    Load(diesel::result::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Connection(e) => e.fmt(f),
            Error::Load(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<diesel::result::Error> for Error {
    fn from(v: diesel::result::Error) -> Self {
        Self::Load(v)
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(v: diesel::ConnectionError) -> Self {
        Self::Connection(v)
    }
}
