use axum::http::HeaderValue;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

use crate::features::folder::repository;
use crate::features::note::repository::PostgresNoteRepository;
use crate::features::note::routes as note_router;
use crate::features::note::service::NoteService;

use crate::features::folder::{
    repository::PostgresFolderRepository, routes as folder_router, service::FolderService,
};

use crate::features::health::routes as health_router;

mod db;
mod error;
mod features;

#[derive(Clone)]
pub struct AppState {
    // AUTO-GENERATED-SERVICE
    pub note_service: NoteService<PostgresNoteRepository>,
    pub folder_service: FolderService<PostgresFolderRepository>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        std::env::var("DB_USER").expect("DB_USER must be set"),
        std::env::var("DB_PSWD").expect("DB_PSWD must be set"),
        std::env::var("DB_URL").expect("DB_URL must be set"),
        std::env::var("DB_PORT").expect("DB_PORT must be set"),
        std::env::var("DB_NAME").expect("DB_NAME must be set")
    );
    println!("DB URL = {}", database_url);
    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    let state = AppState {
        note_service: NoteService {
            repository: PostgresNoteRepository { pool: pool.clone() },
        },
        folder_service: FolderService {
            repository: PostgresFolderRepository { pool: pool.clone() },
        },
    };

    let origins: Vec<HeaderValue> = std::env::var("URL_CORS")
        .unwrap_or_else(|_| "http://localhost:4200".to_string())
        .split(',')
        .filter_map(|s| s.trim().parse::<HeaderValue>().ok())
        .collect();

    println!("{:?}", origins);

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // AUTO-GENERATED-ROUTES
        .nest("/api", folder_router::routes())
        .nest("/api", note_router::routes())
        .nest("/api", health_router::routes())
        .layer(cors)
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
