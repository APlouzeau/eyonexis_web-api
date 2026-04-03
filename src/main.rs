mod db;
mod features;
mod app_state;
mod error;

use app_state::AppState;
use tower_http::cors::{Any, CorsLayer};
use axum::http::HeaderValue;

#[tokio::main]
async fn main() {
    // 1. DB
    let pool = db::connect()
        .await
        .expect("❌ Failed to connect to DB");
    println!("✅ Connected to DB");

    let state = AppState { db: pool };

     // 2. Configuration du CORS
    let frontend_urls = std::env::var("FRONTEND_URLS")
        .unwrap_or_default();
    let allowed_origins: Vec<HeaderValue> = frontend_urls
        .split(',')
        .filter_map(|url| url.trim().parse::<HeaderValue>().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // 3. Router global (compose tous les sous-routers)
    let app = 
        features::health::routes::router()
        .merge(features::notes::routes::router()  // ← Ajoute /notes/*
        .merge(features::folders::routes::router()) // ← Ajoute /folders/*
        .with_state(state))
        .layer(cors);  // ← Partage la DB avec les handlers;
    
    // 4. Serveur
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Server on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
