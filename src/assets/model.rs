use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub address: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct AssetRequest {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(length(min = 5, message = "Description must be at least 5 chars"))]
    pub description: String,

    #[validate]
    pub address: AddressRequest,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct AssetUpdateRequest {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 5))]
    pub description: String,

    #[validate]
    pub address: AddressRequest,
}

#[derive(Debug, Serialize, Deserialize, Validate, sqlx::Type, Clone)]
#[sqlx(type_name = "jsonb")]
pub struct AddressRequest {
    #[validate(length(min = 3))]
    pub street: String,

    #[validate(length(min = 2))]
    pub city: String,

    #[validate(length(equal = 2, message = "State must be exactly 2 chars"))]
    pub state: String,

    #[validate(length(min = 5, max = 5, message = "ZIP must be 5 digits"))]
    pub zip: String,
}
