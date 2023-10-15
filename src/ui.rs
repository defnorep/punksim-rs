use askama::Template;
use rouille::Response;

pub fn web_startup() {
    let template = LayoutTemplate {};
    rouille::start_server("127.0.0.1:3000", move |_request| {
        Response::html(template.render().unwrap())
    });
}

#[derive(Template)]
#[template(path = "layout.html")]
struct LayoutTemplate {}
