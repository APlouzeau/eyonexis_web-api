use axum::{Router, routing::get};
use axum::Extension;
use sqlx::MySqlPool;
mod db;

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

// 3. Fonction main
#[tokio::main]  // ← Cette macro transforme main en async
async fn main() {
    let pool = db::connect().await.expect("Failed to connect to the database");
    println!("Connected to the database successfully!");
    
    // Créer le router
    let app = Router::new()
        .route("/health", get(health))
        .route("/db", get(db_status))
        .layer(Extension(pool));

    // Lancer le serveur
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
