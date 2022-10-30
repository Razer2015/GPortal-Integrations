use api::Transaction;
use chrono::Duration;
use webhook::client::{WebhookClient, WebhookResult};

pub async fn send_donation_webhook(webhook_url: &str, transaction: &Transaction) -> WebhookResult<()> {
    let donator_and_purpose = transaction.get_donator_and_purpose();
    let days = transaction.amount_to_days();
    let donation_day = transaction.time_to_utc();
    let end_date = donation_day + Duration::days(days);

    let client: WebhookClient = WebhookClient::new(webhook_url);
    let webhook_info = client.get_information().await?;
    debug!("webhook: {:?}", webhook_info);

    let vip_management_url = match dotenv::var("VIP_MANAGEMENT_URL") {
        Ok(val) => Some(val),
        Err(_) => None,
    };

    client.send(|message| message
        .username("G-Portal")
        .avatar_url("https://cdn.discordapp.com/attachments/1036028334355795968/1036287507651907674/unknown.png")
        .embed(|embed| embed
            .title("New donation received")
            .description(&donator_and_purpose.1)
            .footer("Webhook by xfileFIN", None)
            .author(&donator_and_purpose.0, vip_management_url.clone(), None)
            .timestamp(&donation_day.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
            .color("15844367")
            .field("Amount", &format!("{} ({} days)", transaction.amount, days), true)
            // .field("Donation days", &days.to_string(), true)
            .field("End date", &format!("<t:{}:R>", end_date.timestamp()), true)
        )
    ).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_webhook_post() {
        dotenv().ok();

        let webhook_path = dotenv::var("DISCORD_DONATION_WEBHOOK").unwrap();

        send_donation_webhook(&webhook_path, &Transaction {
            id: "14500000".to_string(),
            description: "Donation from WebhookTestUser - Purpose: VIP test message".to_string(),
            amount: "1.84 €".to_string(),
            time: "2022-10-29T00:59:33+02:00".to_string()
        }).await.unwrap();
    }
}
