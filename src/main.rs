mod db;
mod features;

use axum::Extension;

#[tokio::main]
async fn main() {
    // 1. DB
    let pool = db::connect()
        .await
        .expect("❌ Failed to connect to DB");
    println!("✅ Connected to DB");
    
    // 2. Router global (compose tous les sous-routers)
    let app = 
        features::health::routes::router()
        .merge(features::notes::routes::router())  // ← Ajoute /notes/*
        .layer(Extension(pool));
    
    // 3. Serveur
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Server on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
