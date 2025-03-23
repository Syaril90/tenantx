use super::model::{Asset, AssetRequest, AssetUpdateRequest};
use super::service;
use crate::AppState;
use crate::shared::response::ApiResponse;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

fn format_validation_errors(errors: &ValidationErrors) -> serde_json::Value {
    json!(errors)
}

pub async fn list_assets(State(state): State<AppState>) -> impl IntoResponse {
    info!("Handling GET /assets");
    let result = service::list_assets(&state.db).await;
    match result {
        Ok(assets) => ApiResponse::success(assets),
        Err(err) => {
            error!(error = %err, "Failed to list assets");
            ApiResponse::error_message(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    }
}

pub async fn get_asset(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    info!(asset_id = %id, "Handling GET /assets/:id");
    match service::get_asset(&state.db, id).await {
        Ok(Some(asset)) => ApiResponse::success(asset),
        Ok(None) => ApiResponse::error_message(StatusCode::NOT_FOUND, "Asset not found"),
        Err(err) => {
            error!(error = %err, asset_id = %id, "Failed to get asset");
            ApiResponse::error_message(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    }
}

pub async fn create_asset(
    State(state): State<AppState>,
    Json(payload): Json<AssetRequest>,
) -> impl IntoResponse {
    info!("Handling POST /assets");

    if let Err(validation_errors) = payload.validate() {
        let formatted_errors = format_validation_errors(&validation_errors);
        error!(?formatted_errors, "Validation failed for create_asset");
        return ApiResponse::error_json(StatusCode::BAD_REQUEST, formatted_errors);
    }

    let asset = Asset {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        address: serde_json::to_value(payload.address).unwrap(),
    };

    match service::create_asset(&state.db, asset).await {
        Ok(asset) => {
            info!(asset_id = %asset.id, "Asset created");
            ApiResponse::created(asset)
        }
        Err(err) => {
            error!(error = %err, "Failed to create asset");
            ApiResponse::error_message(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    }
}

pub async fn update_asset(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AssetUpdateRequest>,
) -> impl IntoResponse {
    info!(asset_id = %id, "Handling PUT /assets/:id");

    if let Err(validation_errors) = payload.validate() {
        let formatted_errors = format_validation_errors(&validation_errors);
        error!(?formatted_errors, "Validation failed for update_asset");
        return ApiResponse::error_json(StatusCode::BAD_REQUEST, formatted_errors);
    }

    let asset = Asset {
        id, // ID is only taken from the path param
        name: payload.name,
        description: payload.description,
        address: serde_json::to_value(payload.address).unwrap(),
    };

    match service::update_asset(&state.db, id, asset).await {
        Ok(Some(asset)) => {
            info!(asset_id = %id, "Asset updated");
            ApiResponse::success(asset)
        }
        Ok(None) => ApiResponse::error_message(StatusCode::NOT_FOUND, "Asset not found"),
        Err(err) => {
            error!(error = %err, asset_id = %id, "Failed to update asset");
            ApiResponse::error_message(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    }
}

pub async fn delete_asset(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    info!(asset_id = %id, "Handling DELETE /assets/:id");

    match service::delete_asset(&state.db, id).await {
        Ok(true) => {
            info!(asset_id = %id, "Asset deleted");
            ApiResponse::success(json!({"message": "Asset deleted successfully"}))
        }
        Ok(false) => ApiResponse::error_message(StatusCode::NOT_FOUND, "Asset not found"),
        Err(err) => {
            error!(error = %err, asset_id = %id, "Failed to delete asset");
            ApiResponse::error_message(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    }
}
