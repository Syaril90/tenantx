use axum::http::StatusCode;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T: Serialize, E: Serialize> {
    Success { data: T },
    Error { error: E },
}

pub struct ApiWithStatus<T: Serialize, E: Serialize> {
    pub status: StatusCode,
    pub body: ApiResponse<T, E>,
}

impl<T: Serialize, E: Serialize> IntoResponse for ApiWithStatus<T, E> {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

impl<T: Serialize> ApiResponse<T, serde_json::Value> {
    pub fn success(data: T) -> ApiWithStatus<T, serde_json::Value> {
        ApiWithStatus {
            status: StatusCode::OK,
            body: ApiResponse::Success { data },
        }
    }

    pub fn created(data: T) -> ApiWithStatus<T, serde_json::Value> {
        ApiWithStatus {
            status: StatusCode::CREATED,
            body: ApiResponse::Success { data },
        }
    }

    pub fn error_json(
        status: StatusCode,
        error: serde_json::Value,
    ) -> ApiWithStatus<T, serde_json::Value> {
        ApiWithStatus {
            status,
            body: ApiResponse::Error { error },
        }
    }

    pub fn error_message(
        status: StatusCode,
        message: impl Into<String>,
    ) -> ApiWithStatus<T, serde_json::Value> {
        Self::error_json(status, serde_json::json!({ "message": message.into() }))
    }
}
