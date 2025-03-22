mod assets;

use axum::{Router, serve};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to database");

    let state = AppState { db: pool };

    let app = Router::new()
        .nest("/assets", assets::routes())
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    serve(listener, app).await.unwrap();
}
