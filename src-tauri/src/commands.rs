use anyhow::Result;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use tauri::State;

use crate::book::Book;
use crate::config::Config;
use crate::extract::{extract_metadata, AcademicMetadata, BookType, FileType};
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

#[tauri::command]
pub async fn add_book(
    state: State<'_, AppState>,
    payload: AddBookPayload,
) -> Result<Book, String> {
    let data_dir: PathBuf = state.config.lock().unwrap().data_dir.clone();
    let mut metadata: MutexGuard<'_, MetadataStore> =
        state.metadata.lock().unwrap();

    let hash_id: String =
        compute_hash(&payload.file_path).map_err(|e| e.to_string())?;

    let (ext_title, ext_authors, ext_year, pages, file_type): (
        String,
        Vec<String>,
        Option<i32>,
        Option<u32>,
        FileType,
    ) = extract_metadata(&payload.file_path).map_err(|e| e.to_string())?;

    let ext_str: &str = match file_type {
        | FileType::Pdf => "pdf",
        | FileType::Epub => "epub",
        | FileType::Mobi => "mobi",
    };

    let target_filename: String = format!("{}.{}", hash_id, ext_str);
    let target_path: PathBuf = data_dir.join("books").join(&target_filename);

    std::fs::create_dir_all(data_dir.join("books"))
        .map_err(|e: Error| e.to_string())?;
    std::fs::copy(&payload.file_path, &target_path)
        .map_err(|e: Error| e.to_string())?;

    let title: String = payload.title.unwrap_or(ext_title);
    let authors: Vec<String> = payload.authors.unwrap_or(ext_authors);
    let year: Option<i32> = payload.year.or(ext_year);
    let genres: Vec<String> = payload.genres.unwrap_or_default();
    let language: String = payload.language.unwrap_or_else(|| "en".to_string());

    let book = Book::new(
        hash_id,
        target_path,
        file_type,
        title,
        authors,
        genres,
        language,
        year,
        pages,
        payload.book_type,
        payload.academic_meta,
    );

    metadata.add_book(book.clone());
    metadata.save(&data_dir).map_err(|e| e.to_string())?;

    Ok(book)
}

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
