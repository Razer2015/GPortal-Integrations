use chrono::{DateTime, Utc, Duration};
use serde_json::json;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::openid::{self, Token};

pub struct GPortalAuth {
    username: String,
    password: String,
    totp_secret: Option<String>,
    token: Option<Token>,
    fetch_time: Option<DateTime<Utc>>,
}

const API_URL: &str = r#"https://auth.g-portal.com/auth/realms/master/protocol/openid-connect/token"#;
const CLIENT_ID: &str = r#"website"#;
const SCOPE: &str = r#"openid email profile gportal"#;

impl GPortalAuth {
    pub fn new(username: String, password: String) -> Self {
        GPortalAuth {
            username,
            password,
            totp_secret: None,
            token: None,
            fetch_time: None,
        }
    }

    pub fn new_with_totp(username: String, password: String, totp_secret: String) -> Self {
        GPortalAuth {
            username,
            password,
            totp_secret: Some(totp_secret),
            token: None,
            fetch_time: None,
        }
    }

    pub fn get_totp_code(totp_secret: &str) -> Result<String, anyhow::Error> {
        let secret = Secret::Encoded(totp_secret.to_string()).to_bytes();
        if secret.is_err() {
            debug!("Trying to get TOTP code with invalid secret");
            return Err(anyhow::anyhow!("TOTP Secret was invalid"));
        }
    
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret.unwrap());
    
        if totp.is_err() {
            debug!("Failed to initialize TOTP");
            return Err(anyhow::anyhow!("Failed to create new TOTP"));
        }
    
        debug!("Generating new TOTP code");
        Ok(totp.unwrap().generate_current()?)
    }

    pub async fn token_by_password(
        username: &str,
        password: &str,
        totp_code: &str,
    ) -> Result<Token, reqwest::Error> {
        let payload = json!({
            "grant_type":"password",
            "client_id":CLIENT_ID,
            "scope":SCOPE,
            "username":username,
            "password":password,
            "rememberMe":"on",
            "totp":totp_code,
        });

        openid::get_token(&API_URL, payload)
            .await
    }

    pub async fn token_by_refreshtoken(
        refresh_token: &str,
    ) -> Result<Token, reqwest::Error> {
        let payload = json!({
            "grant_type":"refresh_token",
            "client_id":CLIENT_ID,
            "refresh_token":refresh_token,
        });

        openid::get_token(&API_URL, payload)
            .await
    }

    pub async fn access_token(&mut self) -> Result<String, anyhow::Error> {
        // Access token is valid
        if self.token.is_some() && !self.is_token_expired() {
            debug!("Access token is valid so using that.");

            return Ok(self.token.as_ref().unwrap().access_token.clone());
        }

        // Access token is invalid but refresh token is valid
        if self.token.is_some() && !self.is_refresh_token_expired() {
            debug!("Access token is invalid but refresh token is valid so using that to fetch a new token.");

            let new_token = GPortalAuth::token_by_refreshtoken(&self.token.as_ref().unwrap().refresh_token).await?;
            self.update_token(new_token);

            return Ok(self.token.as_ref().unwrap().access_token.clone());
        }

        // Neither access token nor refresh token are valid so let's login
        debug!("Neither access token nor refresh token are valid so logging in with {}", self.username);

        let mut totp_code: String = "".to_string();
        if self.totp_secret.is_some() {
            totp_code = GPortalAuth::get_totp_code(&self.totp_secret.as_ref().unwrap())?;
        }

        let token = GPortalAuth::token_by_password(&self.username, &self.password, &totp_code).await?;
        self.update_token(token);

        Ok(self.token.as_ref().unwrap().access_token.clone())
    }

    fn update_token(&mut self, token: Token) {
        self.token = Some(token);
        self.fetch_time = Some(Utc::now());
    }

    fn is_token_expired(&self) -> bool {
        if self.token.is_none() || self.fetch_time.is_none() {
            return true;
        }

        let expire_time = self.fetch_time.unwrap() + Duration::seconds(self.token.as_ref().unwrap().expires_in);
        let date_now = Utc::now();

        date_now >= expire_time
    }

    fn is_refresh_token_expired(&self) -> bool {
        if self.token.is_none() || self.fetch_time.is_none() {
            return true;
        }

        let expire_time = self.fetch_time.unwrap() + Duration::seconds(self.token.as_ref().unwrap().refresh_expires_in);
        let date_now = Utc::now();

        date_now >= expire_time
    }
}
