use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub creado_en: DateTime<Utc>,
    pub modificado_en: DateTime<Utc>,
    pub modificado_por: Option<i32>,
}