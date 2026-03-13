// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // Quelques erreurs courantes pour commencer
    DatabaseError(sqlx::Error),
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // On détermine le Code HTTP et le Message selon le type d'erreur
        let (status, error_message) = match self {
            AppError::DatabaseError(err) => {
                // On pourrait logger l'erreur réelle (err) ici dans la console pour le développeur
                println!("DANGER INTERNAL DB ERROR: {}", err);

                // Mais au client, on renvoie une 500 générique (on ne fuite jamais les infos BDD !)
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne du serveur".to_string())
            }
            AppError::NotFound(message) => {
                (StatusCode::NOT_FOUND, message)
            }
        };

        // On construit une réponse JSON propre
        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// Permet de transformer "magiquement" une sqlx::Error en AppError
impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        AppError::DatabaseError(inner)
    }
}