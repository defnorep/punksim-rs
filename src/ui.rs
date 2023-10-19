pub mod clock_ui;
pub mod population_ui;
pub mod sockets;
pub mod web;

use askama::Template;
use chrono::{DateTime, Utc};

use crate::population::CitizenBundle;

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
}

mod filters {
    use chrono::{DateTime, Utc};

    pub fn date(s: &DateTime<Utc>, format: &str) -> ::askama::Result<String> {
        Ok(s.format(format).to_string())
    }
}
