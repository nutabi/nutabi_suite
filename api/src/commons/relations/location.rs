use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Database-compliant entry for `Nutabi.Location` table
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub record_id: i32,
    pub created_at: DateTime<Local>,
    pub longitude: f64,
    pub latitude: f64,
    pub source: i32,
}
