use askama::Template;
use rouille::Response;

#[derive(Template)]
#[template(path = "layout.html")]
struct LayoutTemplate {}

fn main() {
    rouille::start_server("0.0.0.0:3000", move |request| {
        let template = LayoutTemplate {};
        Response::html(template.render().unwrap())
    });
}
