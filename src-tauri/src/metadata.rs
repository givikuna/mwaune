use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::book::Book;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MetadataStore {
    pub books: Vec<Book>,
}

impl MetadataStore {
    pub fn load(data_dir: &Path) -> Result<Self> {
        let path = data_dir.join("metadata.json");

        if path.exists() {
            Ok(serde_json::from_str(&std::fs::read_to_string(
                &path,
            )?)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, data_dir: &Path) -> Result<()> {
        std::fs::write(
            data_dir.join("metadata.json"),
            serde_json::to_string_pretty(self)?,
        )?;
        Ok(())
    }

    pub fn find_book(&self, id: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.id == id)
    }

    pub fn find_book_mut(&mut self, id: &str) -> Option<&mut Book> {
        self.books.iter_mut().find(|b| b.id == id)
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn remove_books(&mut self, id: &str) -> Option<Book> {
        self.books
            .iter()
            .position(|b| b.id == id)
            .map(|p| self.books.remove(p))
    }
}
