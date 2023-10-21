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
