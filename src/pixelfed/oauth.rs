use crate::oauth;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppDataFromServer {
    id: String,
    name: String,
    website: Option<String>,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

/// Obteined token data from server.
#[derive(Debug, Deserialize, Clone)]
pub struct TokenDataFromServer {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: u64,
    created_at: u64,
}

impl From<AppDataFromServer> for oauth::AppData {
    fn from(val: AppDataFromServer) -> Self {
        oauth::AppData::new(
            val.id,
            val.name,
            val.website,
            Some(val.redirect_uri),
            val.client_id,
            val.client_secret,
        )
    }
}

impl From<TokenDataFromServer> for oauth::TokenData {
    fn from(val: TokenDataFromServer) -> Self {
        oauth::TokenData::new(
            val.access_token,
            val.token_type,
            None,
            Some(val.created_at),
            Some(val.expires_in),
            Some(val.refresh_token),
        )
    }
}
