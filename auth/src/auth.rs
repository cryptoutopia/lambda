use crate::error::Error;
use crate::storage::AuthStorage;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use lambda_runtime::LambdaEvent;
use postgrest::Postgrest;
use serde_json::Value;
use crate::handler::*;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

#[derive(Serialize, Deserialize)]
struct Request {
    email: String,
    password: String,
    token: String,
}

pub trait AuthHandler {
    async fn login(&self, email: &str, password: &str) -> Result<String, Error>;
    async fn logout(&self, token: &str) -> Result<(), Error>;
    async fn register(&self, email: &str, password: &str) -> Result<(), Error>;
    async fn refresh_token(&self, token: &str) -> Result<String, Error                                                                                          >;
    async fn verify_token(&self, token: &str) -> Result<bool, Error>;
    async fn reset_password(&self, email: &str, new_password: &str) -> Result<(), Error>;
}

pub struct StorageAuthenticator<T: AuthStorage> {
    inner: T,
}

// make a new function for the new function to create a new instance of the authenticator
impl<T: AuthStorage> StorageAuthenticator<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait::async_trait]
impl<T: AuthStorage + Send + Sync> AuthHandler for StorageAuthenticator<T> {
    async fn login(&self, email: &str, password: &str) -> Result<String, Error> {
        let user: User = self.inner.get_user_by_email(email).await?;

        if user.password != password {
            return Err(Error::InvalidCredentials);
        }

        let claims = Claims {
            sub: user.id.to_string(),
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("your-256-bit-secret".as_ref()),
        )?;

        Ok(token)
    }

    async fn logout(&self, token: &str) -> Result<(), Error> {
        Ok(())
    }

    async fn register(&self, email: &str, password: &str) -> Result<(), Error> {

        Ok(())
    }

    async fn refresh_token(&self, token: &str) -> Result<String, Error> {
        // 实现 refresh_token 的逻辑
        Ok("new_token".to_string())
    }

    async fn verify_token(&self, token: &str) -> Result<bool, Error> {
        // 实现 verify_token 的逻辑
        Ok(true)
    }

    async fn reset_password(&self, email: &str, new_password: &str) -> Result<(), Error> {
        // 实现 reset_password 的逻辑
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    email: String,
    password: String,
}