pub mod clock_ui;
pub mod sockets;
pub mod web;

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

mod filters {
    use chrono::{DateTime, Utc};

    pub fn date(s: &DateTime<Utc>, format: &str) -> ::askama::Result<String> {
        Ok(s.format(format).to_string())
    }
}
