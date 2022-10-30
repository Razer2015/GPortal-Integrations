use chrono::{DateTime, Utc};
use std::fs::{self};

use crate::{gportal_auth::GPortalAuth, discord};

const CONFIG_PATH: &str = r#"donations_last_fetch.txt"#;

pub struct GPortalDonations {
    auth: GPortalAuth,
    webhook_url: String,
    last_fetch: Option<DateTime<Utc>>,
}

impl GPortalDonations {
    pub fn new(auth: GPortalAuth, webhook_url: String) -> Self {
        GPortalDonations {
            auth,
            webhook_url,
            last_fetch: GPortalDonations::get_last_fetch().unwrap_or(None),
        }
    }

    pub async fn check_new_donations(&mut self) -> Result<(), anyhow::Error> {
        let access_token = self.auth.access_token().await?;
        let donations = api::get_transactions(&access_token).await?.get_donations();

        let last_fetch = if self.last_fetch.is_some() {
            self.last_fetch.unwrap()
        } else {
            Utc::now()
        };
        for donation in donations.iter().rev() {
            if donation.time_to_utc() > last_fetch {
                info!(
                    "New donation: {} - {} - {} ({} days) - {}",
                    donation.id,
                    donation.description,
                    donation.amount_to_currency().to_string(),
                    donation.amount_to_days(),
                    donation.time_to_utc()
                );

                match discord::send_donation_webhook(&self.webhook_url, &donation).await {
                    Ok(_) => (),
                    Err(err) => error!("Error sending the Discord webhook from a Donation: {}", err),
                }
            }
            else {
                debug!(
                    "Old donation: {} - {} - {} ({} days) - {}",
                    donation.id,
                    donation.description,
                    donation.amount_to_currency().to_string(),
                    donation.amount_to_days(),
                    donation.time_to_utc()
                );
            }
        }

        self.last_fetch = Some(Utc::now());
        self.save_last_fetch()?;

        Ok(())
    }

    fn get_last_fetch() -> Result<Option<DateTime<Utc>>, anyhow::Error> {
        let content: String = fs::read_to_string(CONFIG_PATH)?.parse()?;

        let result = content.parse::<DateTime<Utc>>();
        if result.is_err() {
            return Ok(None);
        }

        Ok(Some(result.unwrap()))
    }

    fn save_last_fetch(&self) -> Result<(), anyhow::Error> {
        if self.last_fetch.is_none() {
            return Ok(());
        }

        let last_fetch = self.last_fetch.unwrap();

        fs::write(CONFIG_PATH, last_fetch.to_rfc3339())?;

        Ok(())
    }
}
