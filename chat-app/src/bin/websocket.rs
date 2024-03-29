use std::{time::Duration, sync::{Arc, RwLock}, collections::HashMap};


use tokio::time::sleep;

use futures::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

// 全局唯一的UUID (也可以放到 redis 中)
static NEXT_USERID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

// WSClient
type WSClient = Arc<RwLock<HashMap<usize, SplitSink<WebSocket, Message>>>>;

#[tokio::main]
async fn main() {
    let clients = WSClient::default();
    tokio::spawn(start_axum(clients));

}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// init a background process on the command, and emit periodic events only to the window that used the command
async fn start_axum(clients: WSClient) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // build our application with some routes
    let app = Router::new().with_state(clients)
        // 添加静态文件路径为 webdist
        .nest(
            "",
            get_service(ServeDir::new("./webdist")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .fallback_service(
            get_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
                .handle_error(|error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }),
        )
        // routes are matched from bottom to top, so we have to put `nest` at the
        // top since it matches all routes
        .route("/ws", get(ws_handler))
        .route("/home", get(http_handler))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let user_id = NEXT_USERID.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();

    loop {
        if let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        println!("client sent str: {:?}", t);
                    }
                    Message::Binary(_) => {
                        println!("client sent binary data");
                    }
                    Message::Ping(_) => {
                        println!("socket ping");
                    }
                    Message::Pong(_) => {
                        println!("socket pong");
                    }
                    Message::Close(_) => {
                        println!("client disconnected");
                        return;
                    }
                }
            } else {
                println!("client disconnected");
                return;
            }
        }
    }
}

async fn http_handler() -> Html<&'static str> {

    Html(std::include_str!("../webdist/index.html"))
    // Html(std::include_str!("../../dist/frontend.html"))
}

