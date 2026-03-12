
async fn health() -> &'static str {
    "OK"
}

async fn db_status(Extension(pool): Extension<MySqlPool>) -> &'static str {
    // Tester la connexion avec une vraie requête
    match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => "Database connection successful with a test query",
        Err(_) => "Database connection failed",
    }
}

mod db;
mod routes;
mod handlers;
mod models;

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
        routes::health::router()
        .merge(routes::notes::router())  // ← Ajoute /notes/*
        .layer(Extension(pool));
    
    // 3. Serveur
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Server on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
