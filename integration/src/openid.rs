use reqwest::header::{HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
    pub id_token: String,
    #[serde(alias = "not-before-policy")]
    pub not_before_policy: i64,
    pub session_state: String,
    pub scope: String,
}

const USER_AGENT_STR: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:82.0) Gecko/20100101 Firefox/82.0"#;

pub async fn get_token(path: &str, payload: serde_json::Value) -> Result<Token, reqwest::Error> {
    let client = reqwest::Client::new();
    let k_res = client
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(USER_AGENT, USER_AGENT_STR)
        .form(&payload)
        .send()
        .await?.error_for_status()?;
    k_res.json().await
}
