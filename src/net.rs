use askama::Template;

pub(crate) mod socket_startup;

#[derive(Template)]
#[template(path = "layout.html")]
struct LayoutTemplate {}
