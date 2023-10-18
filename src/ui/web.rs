use super::LayoutTemplate;
use askama::Template;
use rouille::Response;

pub async fn web_startup() {
    rouille::start_server("0.0.0.0:3000", move |_request| {
        let template = LayoutTemplate {};
        Response::html(template.render().unwrap())
    });
}
