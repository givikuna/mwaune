use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    book::Progress,
    extract::{AcademicMetadata, BookType},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddBookPayload {
    pub file_path: PathBuf,
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub language: Option<String>,
    pub year: Option<i32>,
    pub book_type: BookType,
    pub academic_meta: Option<AcademicMetadata>,
} // need to handle cover payload too somehow

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBookPayload {
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub genres: Option<String>,
    pub language: Option<String>,
    pub year: Option<i32>,
    pub book_type: Option<BookType>,
    pub academic_meta: Option<AcademicMetadata>,
    pub progress: Option<Progress>,
}
