mod note;

use axum::{
    routing::{get, post, delete},
    Router
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio;
use crate::note::note::Note;
use crate::note::notes_api ;

#[tokio::main]
async fn main() {
    let notes = Arc::new(Mutex::new(Vec::<Note>::new()));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/notes", get(notes_api::get_notes).post(notes_api::add_note))
        .route("/notes/{id}", delete(notes_api::delete_note))
        .with_state(notes.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, Rust API!"
}