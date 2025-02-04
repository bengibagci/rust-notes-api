use std::sync::{Arc, Mutex};
use axum::extract::{Path, State};
use axum::Json;
use crate::note::note::Note;

type SharedNotes = Arc<Mutex<Vec<Note>>>;

pub async fn add_note(State(notes): State<SharedNotes>, Json(new_note): Json<Note>) -> Json<Vec<Note>> {
    let mut notes = notes.lock().unwrap();
    notes.push(new_note);
    Json(notes.clone())
}

pub async fn delete_note(State(notes): State<SharedNotes>, Path(id): Path<u32>) -> Json<Vec<Note>> {
    let mut notes = notes.lock().unwrap();
    notes.retain(|note| note.id != id);
    Json(notes.clone())
}

pub async fn get_notes(State(notes): State<SharedNotes>) -> Json<Vec<Note>> {
    let notes = notes.lock().unwrap();
    Json(notes.clone())
}
