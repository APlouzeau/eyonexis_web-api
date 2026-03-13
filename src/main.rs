mod db;
mod features;
mod app_state;

use app_state::AppState;

#[tokio::main]
async fn main() {
    // 1. DB
    let pool = db::connect()
        .await
        .expect("❌ Failed to connect to DB");
    println!("✅ Connected to DB");

    let state = AppState { db: pool };
    
    // 2. Router global (compose tous les sous-routers)
    let app = 
        features::health::routes::router()
        .merge(features::notes::routes::router()  // ← Ajoute /notes/*
        .with_state(state));  // ← Partage la DB avec les handlers;
    
    // 3. Serveur
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Server on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
