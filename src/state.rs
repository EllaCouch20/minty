use chrono::{Local, DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::BITCOIN_PRICE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintyContract {
    pub prediction: f64,
    pub expected_amt: f64,
    pub deposited: f64,
    pub minimum: f64,
    pub btc_price: f64,
    pub timestamp: DateTime<Utc>,
    pub address: String,
}

impl MintyContract {
    pub fn empty() -> Self {
        MintyContract {
            prediction: 0.0,
            expected_amt: 0.0,
            deposited: 0.0,
            minimum: 0.0,
            btc_price: BITCOIN_PRICE,
            timestamp: Utc::now(),
            address: String::new(),
        }
    }

    pub fn test() -> Self {
        MintyContract {
            prediction: 1_000_000.0,
            expected_amt: 800_000.0,
            deposited: 100_000.0,
            minimum: 1_200.0,
            btc_price: BITCOIN_PRICE,
            timestamp: Utc::now(),
            address: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyContracts(pub Vec<MintyContract>);