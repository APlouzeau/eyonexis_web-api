use sqlx::MySqlPool;
use dotenvy::dotenv;

pub async fn connect() -> sqlx::Result<MySqlPool> {
    dotenv().ok();
    let db_url = std::env::var("DB_URL").expect("DB_URL must be set");
    let db_user = std::env::var("DB_USER").expect("DB_USER must be set");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");

    let database_url = format!("mysql://{}:{}@{}/{}", db_user, db_password, db_url, db_name);

    MySqlPool::connect(&database_url).await
}