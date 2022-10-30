use currency_rs::{Currency};
use regex::Regex;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsGrid {
    pub grid: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub description: String,
    pub amount: String,
    pub time: String,
}

impl TransactionsGrid {
    pub fn get_donations(&self) -> Vec<Transaction> {
        let results: Vec<Transaction> = self.grid
            .iter()
            .filter_map(|p| match p[1].starts_with("Donation from") {
                true => {
                    Some(Transaction {
                        id: p[0].to_string(),
                        description: p[1].to_string(),
                        amount: p[2].to_string(),
                        time: p[3].to_string(),
                    })
                },
                false => None,
            })
            .collect();

        results
    }
}

impl Transaction {
    pub fn get_donator_and_purpose(&self) -> (String, String) {
        let re = Regex::new(r"Donation from (.*) - Purpose: (.*)").unwrap();
        let capture_result = re.captures(&self.description);

        if capture_result.is_none() {
            return ("Unknown".to_string(), self.description.to_string());
        }
        
        let caps = capture_result.unwrap();
        let donator = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let purpose = caps.get(2).map_or("", |m| m.as_str()).to_string();

        (donator, purpose)
    }

    pub fn time_to_utc(&self) -> DateTime<Utc> {
        self.time.parse::<DateTime<Utc>>().unwrap()
    }

    pub fn amount_to_currency(&self) -> Currency {
        Currency::new_string(&self.amount, None).unwrap()
    }

    /**
     * LSD specific donation pricing
     * TODO: Make this configurable via config
    */
    pub fn amount_to_days(&self) -> i64 {
        let currency = self.amount_to_currency();
        let donation_amount = currency.value();

        let days: f64;
        if donation_amount >= 10.0 {
            days = donation_amount / 10.0 * 90.0;
        }
        else if donation_amount >= 8.0 {
            days = donation_amount / 8.0 * 60.0;
        }
        else {
            days = donation_amount / 5.0 * 30.0;
        }

        days.round() as i64
    }
}
