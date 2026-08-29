use sha2::{Digest, Sha256};

use crate::{error::AppError, features::auth::repository::AuthRepository};

#[derive(Clone)]
pub struct AuthService<R: AuthRepository> {
    pub repository: R,
}

impl<R: AuthRepository> AuthService<R> {
    pub async fn verify_token(&self, token_received: String) -> Result<(), AppError> {
        let mut hasher = Sha256::new();
        hasher.update(token_received);
        let token_received_hashed = hasher.finalize();
        self.repository
            .verify_token_hashed(hex::encode(token_received_hashed))
            .await?
            .ok_or_else(|| AppError::Unauthorized("Non autorisé".to_string()))?;
        Ok(())
    }
}
