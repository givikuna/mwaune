use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum HighlightColor {
    Yellow,
    Green,
    Blue,
    Pink,
    Orange,
    Red,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub page: u32,
    pub text: String,
    pub color: HighlightColor,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Note {
    pub fn new(page: u32, text: String, color: HighlightColor) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            page,
            text,
            color,
            created_at: now,
            updated_at: now,
        }
    }
}
