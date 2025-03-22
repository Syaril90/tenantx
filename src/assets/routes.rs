use super::handlers::*;
use crate::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_assets).post(create_asset))
        .route(
            "/:id",
            get(get_asset).put(update_asset).delete(delete_asset),
        )
}
