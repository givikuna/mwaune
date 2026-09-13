pub mod book_payload;
pub mod filter_options;
pub mod read_status;

pub use book_payload::*;
pub use filter_options::*;
pub use read_status::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum FileType {
    Pdf,
    Epub,
    Mobi,
}

impl FileType {
    pub fn ext(self) -> &'static str {
        match self {
            | Self::Pdf => "pdf",
            | Self::Epub => "epub",
            | Self::Mobi => "mobi",
        }
    }

    pub fn from_ext(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            | "pdf" => Some(Self::Pdf),
            | "epub" => Some(Self::Epub),
            | "mobi" => Some(Self::Mobi),
            | _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BookType {
    Book,
    Paper,
    AcademicPaper,
}
