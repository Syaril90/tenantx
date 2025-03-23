use super::handlers;
use crate::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_assets).post(handlers::create_asset))
        .route(
            "/:id",
            get(handlers::get_asset)
                .put(handlers::update_asset)
                .delete(handlers::delete_asset),
        )
}
