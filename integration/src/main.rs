#[macro_use]
extern crate log;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use dotenv::dotenv;
use std::time::Duration;
use tokio::time::sleep;

mod discord;
mod gportal_auth;
mod gportal_donations;
mod logging;
mod openid;

fn get_timezone() -> Tz {
    let timezone = dotenv::var("CHRONO_TIMEZONE").unwrap_or("Europe/Helsinki".to_string());
    timezone.parse().unwrap()
}

fn get_time_after_duration(duration: u64) -> String {
    let tz: Tz = get_timezone();
    let now: DateTime<Tz> = Utc::now().with_timezone(&tz);
    let time = now + chrono::Duration::milliseconds(duration as i64);

    time.format("%Y-%m-%dT%H:%M:%S").to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging::init_logging();

    info!("G-Portal Integrations starting");
    info!("Using time zone: {}", get_timezone().name());

    let username = dotenv::var("GPORTAL_USERNAME").unwrap();
    let password = dotenv::var("GPORTAL_PASSWORD").unwrap();
    let totp_secret = dotenv::var("TOTP_SECRET").unwrap_or("".to_string());
    let donation_interval: u64 = dotenv::var("DONATION_INTERVAL")
        .map(|var| var.parse::<u64>())
        .unwrap_or(Ok(900_000))
        .unwrap();

    let auth_client = if totp_secret.is_empty() {
        gportal_auth::GPortalAuth::new(username, password)
    } else {
        gportal_auth::GPortalAuth::new_with_totp(username, password, totp_secret)
    };

    let donation_webhook = dotenv::var("DISCORD_DONATION_WEBHOOK").unwrap_or("".to_string());
    if !donation_webhook.is_empty() {
        let mut donations = gportal_donations::GPortalDonations::new(auth_client, donation_webhook);
        loop {
            if let Err(err) = donations.check_new_donations().await {
                error!("Error while polling new donations: {}", err);
            }

            info!(
                "Polling for new donations done, next poll at {}",
                get_time_after_duration(donation_interval)
            );
            sleep(Duration::from_millis(donation_interval)).await;
        }
    }
    else {
        info!("Skipping donation fetching because donation webhook is empty. Please add the Discord webhook in the 'DISCORD_DONATION_WEBHOOK' environment variable if you want to get notified from new donations.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_totp_secret() {
        let totp_secret = dotenv::var("TOTP_SECRET").unwrap_or("".to_string());

        let code_result = gportal_auth::GPortalAuth::get_totp_code(&totp_secret);

        if code_result.is_err() {
            println!(
                "Failed to generate TOTP code | {}",
                code_result.err().unwrap()
            );
            return;
        }

        println!("{}", code_result.unwrap())
    }

    #[tokio::test]
    async fn test_get_access_token_by_password() {
        let username = dotenv::var("GPORTAL_USERNAME").unwrap_or("".to_string());
        let password = dotenv::var("GPORTAL_PASSWORD").unwrap_or("".to_string());

        let token = gportal_auth::GPortalAuth::token_by_password(&username, &password, "").await;

        if token.is_err() {
            println!("Failed to fetch access token | {}", token.err().unwrap());
            return;
        }

        let access_token = token.as_ref().map(|res| res.access_token.clone());
        let refresh_token = token.as_ref().map(|res| res.refresh_token.clone());

        println!("Access token: {}", access_token.unwrap());
        println!("");
        println!("Refresh token: {}", refresh_token.unwrap());
    }

    #[tokio::test]
    async fn test_get_access_token_by_refreshtoken() {
        let refresh_token = dotenv::var("GPORTAL_REFRESH_TOKEN").unwrap_or("".to_string());

        let token = gportal_auth::GPortalAuth::token_by_refreshtoken(&refresh_token).await;

        if token.is_err() {
            println!("Failed to fetch access token | {}", token.err().unwrap());
            return;
        }

        let access_token = token.as_ref().map(|res| res.access_token.clone());
        let refresh_token = token.as_ref().map(|res| res.refresh_token.clone());

        println!("Access token: {}", access_token.unwrap());
        println!("");
        println!("Refresh token: {}", refresh_token.unwrap());
    }
}
