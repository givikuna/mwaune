use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::extract::{AcademicMetadata, BookType, FileType};
use crate::notes::Note;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress {
    pub page: u32,
    pub percentage: f32,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub genres: Vec<String>,
    pub language: String,
    pub year: Option<i32>,
    pub pages: Option<u32>,
    pub cover_path: Option<PathBuf>,
    pub file_path: PathBuf,
    pub file_type: FileType,
    pub book_type: BookType,
    pub read: bool,
    pub progress: Progress,
    pub notes: Vec<Note>,
    pub academic_meta: Option<AcademicMetadata>,
}

impl Book {
    pub fn new(
        id: String,
        file_path: PathBuf,
        file_type: FileType,
        title: String,
        authors: Vec<String>,
        genres: Vec<String>,
        language: String,
        year: Option<i32>,
        pages: Option<u32>,
        book_type: BookType,
        academic_meta: Option<AcademicMetadata>,
    ) -> Self {
        Self {
            id,
            title,
            authors,
            genres,
            language,
            year,
            pages,
            cover_path: None,
            file_path,
            file_type,
            book_type,
            read: false,
            progress: Progress {
                page: 0,
                percentage: 0.0,
                updated_at: 0,
            },
            notes: vec![],
            academic_meta,
        }
    }

    /*
    pub fn update_from_payload(&mut self, payload: crate::commands::UpdateBookPayload) {
        if let Some(title) = payload.title {
            self.title = title;
        }
        if let Some(authors) = payload.authors {
            self.authors = authors;
        }
        if let Some(genres) = payload.genres {
            self.genres = genres;
        }
        if let Some(language) = payload.language {
            self.language = language;
        }
        if let Some(year) = payload.year {
            self.year = Some(year);
        }
        if let Some(book_type) = payload.book_type {
            self.book_type = book_type;
        }
        if let Some(academic_meta) = payload.academic_meta {
            self.academic_meta = Some(academic_meta);
        }
        if let Some(read) = payload.read {
            self.read = read;
        }
    }
    */
}
