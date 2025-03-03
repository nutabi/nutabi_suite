use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Database-compliant entry for `Nutabi.DataSource` table
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataSource {
    pub source_id: i32,
    pub created_at: DateTime<Local>,
    pub last_updated_at: DateTime<Local>,
    pub name: String,
    pub description: Option<String>,
    pub access_key: String,
}
