use anyhow::{anyhow, Result};
use epub::doc::{EpubDoc, MetadataItem};
use lopdf::{Document, Object};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

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

pub fn extract_metadata(
    path: &Path,
) -> Result<(
    String,
    Vec<String>,
    Option<i32>,
    Option<u32>,
    FileType,
)> {
    let ext: String = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        | "epub" => extract_epub_metadata(path),
        | "pdf" => extract_pdf_metadata(path),
        | _ => Err(anyhow!("unsupported file type")),
    }
}

fn extract_epub_metadata(
    path: &Path,
) -> Result<(
    String,
    Vec<String>,
    Option<i32>,
    Option<u32>,
    FileType,
)> {
    let doc: EpubDoc<BufReader<File>> = EpubDoc::new(path)?;

    let title: String = doc
        .mdata("title")
        .map(|m: &MetadataItem| m.value.clone())
        .unwrap_or_else(|| "Unknown Title".to_string());

    let author: String = doc
        .mdata("creator")
        .map(|m: &MetadataItem| m.value.clone())
        .unwrap_or_else(|| "Unknown Author".to_string());

    let pages: Option<u32> = Some(doc.get_num_chapters() as u32);

    Ok((title, vec![author], None, pages, FileType::Epub))
}

fn extract_pdf_metadata(
    path: &Path,
) -> Result<(
    String,
    Vec<String>,
    Option<i32>,
    Option<u32>,
    FileType,
)> {
    let doc: Document = Document::load(path)?;
    let pages: u32 = doc.get_pages().len() as u32;

    let mut title: String = "Unknown Title".to_string();
    let mut authors: Vec<String> = vec!["Unknown Author".to_string()];

    if let Some(info_id) = doc
        .trailer
        .get(b"Info")
        .and_then(|obj| obj.as_reference())
        .ok()
    {
        if let Ok(info_dict) = doc.get_dictionary(info_id) {
            if let Ok(t_bytes) = info_dict
                .get(b"Title")
                .and_then(|obj: &Object| obj.as_str())
            {
                if !t_bytes.is_empty() {
                    title = String::from_utf8_lossy(t_bytes).into_owned();
                }
            }
            if let Ok(a_bytes) = info_dict
                .get(b"Author")
                .and_then(|obj: &Object| obj.as_str())
            {
                if !a_bytes.is_empty() {
                    authors =
                        vec![String::from_utf8_lossy(a_bytes).into_owned()];
                }
            }
        }
    }

    Ok((title, authors, None, Some(pages), FileType::Pdf))
}

fn generate_cover(
    _id: &str,
    _file_path: &Path,
    _data_dir: &Path,
) -> Result<Option<PathBuf>> {
    Ok(None)
}
