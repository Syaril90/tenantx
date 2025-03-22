use super::model::{Address, Asset, AssetInput};
use super::service;
use crate::AppState;
use axum::extract::Query;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

fn parse_address(value: &serde_json::Value) -> Address {
    serde_json::from_value(value.clone()).unwrap_or(Address {
        street: "".into(),
        city: "".into(),
        state: "".into(),
        zip: "".into(),
    })
}
pub async fn list_assets(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let page: i64 = params.get("page").and_then(|v| v.parse().ok()).unwrap_or(1);
    let per_page: i64 = params
        .get("per_page")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let offset = (page - 1) * per_page;

    match service::list_assets(&state.db, per_page, offset).await {
        Ok((assets, total)) => {
            let total_pages = (total as f64 / per_page as f64).ceil() as i64;
            let assets: Vec<_> = assets
                .into_iter()
                .map(|a| {
                    let addr = parse_address(&a.address);
                    json!({
                        "id": a.id,
                        "name": a.name,
                        "description": a.description,
                        "address": addr
                    })
                })
                .collect();

            Json(json!({
                "data": assets,
                "meta": {
                    "total": total,
                    "total_pages": total_pages,
                    "current_page": page,
                    "per_page": per_page
                }
            }))
            .into_response()
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn get_asset(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match service::get_asset(&state.db, id).await {
        Ok(Some(asset)) => {
            let addr = parse_address(&asset.address);
            Json(json!({
                "id": asset.id,
                "name": asset.name,
                "description": asset.description,
                "address": addr
            }))
            .into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Asset not found".to_string()).into_response(),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn create_asset(
    State(state): State<AppState>,
    Json(payload): Json<AssetInput>,
) -> impl IntoResponse {
    let asset = Asset {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        address: serde_json::to_value(payload.address).unwrap(),
    };

    match service::create_asset(&state.db, asset).await {
        Ok(asset) => {
            let addr = parse_address(&asset.address);
            (
                StatusCode::CREATED,
                Json(json!({
                    "id": asset.id,
                    "name": asset.name,
                    "description": asset.description,
                    "address": addr
                })),
            )
                .into_response()
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn update_asset(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AssetInput>,
) -> impl IntoResponse {
    let updated_asset = Asset {
        id,
        name: payload.name,
        description: payload.description,
        address: serde_json::to_value(payload.address).unwrap(),
    };

    match service::update_asset(&state.db, id, updated_asset).await {
        Ok(Some(asset)) => {
            let addr = parse_address(&asset.address);
            Json(json!({
                "id": asset.id,
                "name": asset.name,
                "description": asset.description,
                "address": addr
            }))
            .into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Asset not found".to_string()).into_response(),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn delete_asset(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service::delete_asset(&state.db, id).await {
        Ok(true) => (StatusCode::NO_CONTENT, "").into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Asset not found".to_string()).into_response(),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}
