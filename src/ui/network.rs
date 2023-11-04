use crate::ui::LayoutTemplate;
use askama::Template;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use flume::Receiver;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter};

async fn index() -> impl IntoResponse {
    let template = LayoutTemplate {};
    let html = template.render().unwrap();

    (StatusCode::OK, Html(html).into_response())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let rx = state.rx.clone();

    tracing::info!("ws client connected: {}", addr.ip());

    ws.on_upgrade(move |socket| handle_socket(socket, addr, rx))
}

async fn handle_socket(mut socket: WebSocket, addr: SocketAddr, rx: Receiver<String>) {
    tracing::info!("ws client connected: {}:{}", addr.ip(), addr.port());

    loop {
        let msg = rx.recv_async().await.unwrap();
        tracing::debug!("msg -> {}:{} ({} bytes)", addr.ip(), addr.port(), msg.len());
        socket.send(Message::Text(msg)).await.unwrap();
    }
}

struct AppState {
    rx: Receiver<String>,
}

fn app(rx: Receiver<String>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/ws", get(ws_handler))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(Arc::new(AppState { rx }))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

pub async fn network_startup(rx: Receiver<String>) {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("http listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app(rx).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
