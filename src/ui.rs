pub mod census_ui;
pub mod clock_ui;
pub mod individuals_ui;
pub mod sockets;
pub mod web;

use crate::population::CitizenBundle;
use askama::Template;
use chrono::{DateTime, Utc};

#[derive(Template)]
#[template(path = "partials/clock.html")]
struct ClockTemplate {
    datetime: DateTime<Utc>,
}

#[derive(Template)]
#[template(path = "layout.html")]
struct LayoutTemplate {}

#[derive(Template)]
#[template(path = "partials/individuals.html")]
struct IndividualsTemplate {
    individuals: Vec<CitizenBundle>,
    reference: DateTime<Utc>,
}

#[derive(Template)]
#[template(path = "partials/census.html")]
struct CensusTemplate {
    pub total: u32,
    pub human: u32,
    pub android: u32,
    pub male: u32,
    pub female: u32,
    pub non_binary: u32,
    pub ungendered: u32,
    pub living: u32,
    pub deceased: u32,
    pub missing: u32,
}

mod filters {
    use chrono::{DateTime, Utc};

    pub fn date(s: &DateTime<Utc>, format: &str) -> ::askama::Result<String> {
        Ok(s.format(format).to_string())
    }

    pub fn age(comparison: &DateTime<Utc>, reference: &DateTime<Utc>) -> ::askama::Result<String> {
        let age = reference.years_since(comparison.to_owned()).unwrap();
        Ok(format!("{}", age))
    }
}
