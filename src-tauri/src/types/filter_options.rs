use serde::{Deserialize, Serialize};

use crate::{extract::BookType, types::read_status::ReadStatus};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FilterOptions {
    pub query: Option<String>,
    pub genres: Option<Vec<String>>,
    pub language: Option<String>,
    pub book_type: Option<BookType>,
    pub read_status: Option<ReadStatus>,
    pub year_range: Option<(i32, i32)>,
    pub page_range: Option<(u32, u32)>,
}
