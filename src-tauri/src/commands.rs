use anyhow::Result;
use std::path::Path;
use std::sync::Mutex;
use tauri::State;

use crate::book::Book;
use crate::config::Config;
use crate::extract::{
    extract_metadata, generate_cover, AcademicMetadata, BookType,
};
use crate::metadata::MetadataStore;
use crate::notes::{HighlightColor, Note};
// use crate::search::filter_books;
use crate::types::{
    book_payload::{AddBookPayload, UpdateBookPayload},
    filter_options::FilterOptions,
    read_status::ReadStatus,
};

pub struct AppState {
    pub config: Mutex<Config>,
    pub metadata: Mutex<MetadataStore>,
}

pub fn compute_hash(path: &Path) -> Result<String, anyhow::Error> {
    let data = std::fs::read(path)?;
    let hash = blake3::hash(&data);
    Ok(hash.to_hex().to_string())
}

#[tauri::command]
pub async fn get_books(
    state: State<'_, AppState>,
    filters: Option<FilterOptions>,
) -> Result<Vec<Book>, String> {
    let books = &state.metadata.lock().unwrap().books;

    let filtered = if let Some(_f) = filters {
        // To-Do filter_books
        // filter_books(books, &f)
        books.clone()
    } else {
        books.clone()
    };

    Ok(filtered)
}

/*
#[tauri::command]
pub async fn add_book(
    state: State<'_, AppState>,
    payload: AddBookPayload,
) -> Result<Book, String> {
    //
}
*/

/*
pub async fn update_book(
    state: State<'_, AppState>,
    id: String,
    payload: UpdateBookPayload,
) -> Result<(), String> {
    //
}
*/

#[tauri::command]
pub async fn delete_book(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let config = state.config.lock().unwrap();
    let data_dir = &config.data_dir;
    let mut metadata = state.metadata.lock().unwrap();

    if let Some(book) = metadata.remove_books(&id) {
        // Remove files
        let book_path = data_dir
            .join("books")
            .join(book.file_path.file_name().unwrap());
        let _ = std::fs::remove_file(book_path);
        if let Some(cover) = book.cover_path {
            let _ = std::fs::remove_file(
                data_dir.join("covers").join(cover.file_name().unwrap()),
            );
        }
        metadata.save(data_dir).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Book not found".to_string())
    }
}

#[tauri::command]
pub async fn add_note(
    state: State<'_, AppState>,
    book_id: String,
    page: u32,
    text: String,
    color: HighlightColor,
) -> Result<Note, String> {
    let config = state.config.lock().unwrap();
    let data_dir = &config.data_dir;
    let mut metadata = state.metadata.lock().unwrap();

    if let Some(book) = metadata.find_book_mut(&book_id) {
        let note = Note::new(page, text, color);
        book.notes.push(note.clone());
        metadata.save(data_dir).map_err(|e| e.to_string())?;
        Ok(note)
    } else {
        Err("Book not found".to_string())
    }
}

#[tauri::command]
pub async fn update_progress(
    state: State<'_, AppState>,
    book_id: String,
    page: u32,
) -> Result<(), String> {
    let config = state.config.lock().unwrap();
    let data_dir = &config.data_dir;
    let mut metadata = state.metadata.lock().unwrap();

    if let Some(book) = metadata.find_book_mut(&book_id) {
        let total_pages = book.pages.unwrap_or(1);
        book.progress.page = page;
        book.progress.percentage = if total_pages > 0 {
            (page as f32) / (total_pages as f32)
        } else {
            0.0
        };
        book.progress.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        metadata.save(data_dir).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Book not found".to_string())
    }
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}

#[tauri::command]
pub async fn update_config(
    state: State<'_, AppState>,
    new_config: Config,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    *config = new_config;
    config.save().map_err(|e| e.to_string())?;
    Ok(())
}
