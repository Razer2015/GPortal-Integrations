#[macro_use] extern crate log;

pub mod models;

use http::{header::USER_AGENT, StatusCode};
pub use models::*;

const API_URL: &str = r#"https://www.g-portal.com"#;
const USER_AGENT_STR: &str =
    r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:82.0) Gecko/20100101 Firefox/82.0"#;

pub async fn get_transactions(access_token: &str) -> Result<TransactionsGrid, anyhow::Error> {
    let client = reqwest::Client::new();

    let page = 0;
    let page_size = 25;
    let sort_column = "-"; // - (default) | id | description | activity | time
    let sort_order = "-"; // - (default) | asc (ascending) | desc (descending)

    let res = client
        .get(format!(
            "{}/eur/profile/transactions/{}/{}/{}/{}",
            API_URL, page, page_size, sort_column, sort_order
        ))
        .header(USER_AGENT, USER_AGENT_STR)
        .header("Host", "www.g-portal.com")
        .header("Origin", "https://www.g-portal.com")
        .header(
            "Referer",
            "https://www.g-portal.com/eur/profile/payments/transactions",
        )
        .header("X-Application-Name", "VueJS")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Cookie", format!("gp_at={}", access_token))
        .send()
        .await?;

    let status = res.status();

    let data_str = res.text().await?;
    //println!("{}", data_str);

    if status != StatusCode::OK {
        debug!("Failed to fetch transactions from GPortal");
        return Err(anyhow::anyhow!(data_str));
    }

    let data: TransactionsGrid = serde_json::from_str(&data_str)?;
    trace!("TransactionsGrid: {:#?}", data);

    debug!("Fetched {} transactions from GPortal", data.grid.len());

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_transactions_grid_struct() {
        dotenv().ok();

        let data_str = r#"
        {
            "grid": [
                [
                    "14500004",
                    "Donation from xfileFIN - Purpose: I neeed VIIIIP!!!",
                    "150.00 €",
                    "2022-10-30T03:00:00+02:00"
                ],
                [
                    "14500003",
                    "Gamecloud Basic - Gamecloud Basic",
                    "-32.70 €",
                    "2022-10-29T20:20:05+02:00"
                ],
                [
                    "14500002",
                    "Donation from xfileFIN@xfileFIN.com - Purpose: VIP for xfileFIN",
                    "5.00 €",
                    "2022-10-29T20:10:05+02:00"
                ],
                [
                    "14500001",
                    "Donation from T3stingMan - Purpose: soldiername: xfileFIN\nDiscord tag: xfileFIN#2811",
                    "11.84 €",
                    "2022-10-28T21:50:01+02:00"
                ],
                [
                    "14500000",
                    "Donation from poorGuy - Purpose: PoorGuy",
                    "1.49 €",
                    "2022-10-01T21:50:01+02:00"
                ]
            ],
            "total": 112
        }"#;

        let data: TransactionsGrid = serde_json::from_str(&data_str).unwrap();
        println!("TransactionsGrid: {:#?}", data);

        println!("\nDonations:");
        let donations = data.get_donations();
        for donation in &donations {
            println!(
                "{} - {} - {} ({} days) - {}",
                donation.id,
                donation.description,
                donation.amount_to_currency().to_string(),
                donation.amount_to_days(),
                donation.time_to_utc()
            );
        }
        println!("");
    }
}
