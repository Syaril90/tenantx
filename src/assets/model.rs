use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub address: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetInput {
    pub name: String,
    pub description: String,
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "jsonb")]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}
