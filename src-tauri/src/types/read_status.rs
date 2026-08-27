use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReadStatus {
    Read,
    Unread,
    InProgress,
}
