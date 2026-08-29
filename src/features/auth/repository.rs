use sqlx::PgPool;

use crate::features::auth::model::AuthRequest;

#[derive(Clone)]
pub struct PostgresAuthRepository {
    pub pool: PgPool,
}

impl AuthRepository for PostgresAuthRepository {
    fn verify_token_hashed(
        &self,
        token_received: String,
    ) -> impl std::future::Future<Output = Result<Option<AuthRequest>, sqlx::Error>> + Send {
        async move {
            let token_registered = sqlx::query_as!(
                AuthRequest,
                r#"
                UPDATE device_tokens
                SET last_connected_at = now()
                WHERE token_hash = $1
                RETURNING
                token_hash
                "#,
                token_received
            )
            .fetch_optional(&self.pool)
            .await?;
            Ok(token_registered)
        }
    }
}

pub trait AuthRepository {
    fn verify_token_hashed(
        &self,
        token_received: String,
    ) -> impl std::future::Future<Output = Result<Option<AuthRequest>, sqlx::Error>> + Send;
}
