use crate::{
    module::user::model::{UserClaim, UserRefreshClaim},
    util::{config::CONFIG, error::AppError},
};

use axum::{RequestPartsExt, extract::FromRequestParts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use once_cell::sync::Lazy;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = CONFIG.auth.secret.clone();
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub fn sign(user_claim: UserClaim) -> Result<String, AppError> {
    let token = encode(&Header::default(), &user_claim, &KEYS.encoding)
        .map_err(|_| AppError::InvalidToken)?;
    Ok(token)
}

pub fn sign_resfresh(user_refresh_claim: UserRefreshClaim) -> Result<String, AppError> {
    let token = encode(&Header::default(), &user_refresh_claim, &KEYS.encoding)
        .map_err(|_| AppError::InvalidToken)?;
    Ok(token)
}

impl<S> FromRequestParts<S> for UserClaim
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 从请求头中提取 token
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InvalidToken)?;
        // 编码 token 获取用户访问令牌
        let token_data =
            decode::<UserClaim>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl<S> FromRequestParts<S> for UserRefreshClaim
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 从请求头中提取 token
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InvalidToken)?;
        // 编码 token 获取用户刷新令牌
        let token_data =
            decode::<UserRefreshClaim>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
