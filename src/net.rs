use askama::Template;
use rouille::Response;

pub(crate) mod socket_startup;

#[derive(Template)]
#[template(path = "layout.html")]
struct LayoutTemplate {}

pub async fn web_startup() {
    rouille::start_server("0.0.0.0:3000", move |request| {
        let template = LayoutTemplate {};
        Response::html(template.render().unwrap())
    });
}
