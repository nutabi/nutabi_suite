use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Database-compliant entry for `Nutabi.DataSource` table
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataSource {
    pub source_id: i32,
    pub created_at: NaiveDateTime,
    pub last_updated_at: NaiveDateTime,
    pub name: String,
    pub description: Option<String>,
}
