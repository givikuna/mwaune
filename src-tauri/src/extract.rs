use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AcademicMetadata {
    pub institution: Option<String>,
    pub doi: Option<String>,
    pub journal: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum BookType {
    Book,
    Paper,
    AcademicPaper,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum FileType {
    Pdf,
    Epub,
    Mobi,
}

// to-do
pub fn extract_metadata(
    _path: &Path,
) -> Result<(
    String,
    Vec<String>,
    Option<i32>,
    Option<u32>,
    FileType,
)> {
    Ok((
        "Unknown Title".to_string(),
        vec!["Unknown Author".to_string()],
        None,
        None,
        FileType::Pdf,
    ))
}

// to-do
pub fn generate_cover(_id: &str, _file_path: &Path, _data_dir: &Path) -> Result<Option<PathBuf>> {
    Ok(None)
}
