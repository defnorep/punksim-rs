use super::LayoutTemplate;
use askama::Template;
use rouille::Response;

pub async fn web_startup() {
    rouille::start_server("0.0.0.0:3000", move |request| {
        // public assets are in root/public
        if let Some(request) = request.remove_prefix("/public") {
            return rouille::match_assets(&request, "public");
        }

        let template = LayoutTemplate {};
        Response::html(template.render().unwrap())
    });
}
