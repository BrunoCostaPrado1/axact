use std::sync::{ Arc, Mutex };

use axum::{ Server, Router, routing::get, extract::State, Json, response::{ IntoResponse, Html } };
use sysinfo::{ System, SystemExt, CpuExt };

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}
#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    "Hello, World!";
    Html(include_str!("index.html"))
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    //FIXME:Fell bad
    sys.refresh_cpu();

    let v: Vec<_> = sys
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect();

    Json(v)
}