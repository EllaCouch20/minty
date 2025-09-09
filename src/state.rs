use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use pelican_ui::events::Event;

use crate::Context;
use crate::BITCOIN_PRICE;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MintyContract {
    pub prediction: f64,
    pub expected_amt: f64,
    pub deposited: f64,
    pub minimum: f64,
    pub btc_price: f64,
    pub timestamp: DateTime<Utc>,
    pub address: String,
    pub accepted: bool,
    pub variant: ContractType,
    pub is_risky: bool,
    pub self_published: bool,
    pub matched_with: Option<Box<MintyContract>>,
    pub accepted_contract: Option<Box<MintyContract>>,
}

impl MintyContract {
    pub fn empty() -> Self {
        MintyContract {
            prediction: 500_000.0,
            expected_amt: 0.0,
            deposited: 0.0,
            minimum: 0.0,
            btc_price: BITCOIN_PRICE,
            timestamp: Utc::now(),
            address: String::new(),
            accepted: false,
            variant: ContractType::Unknown,
            is_risky: false,
            self_published: false,
            matched_with: None,
            accepted_contract: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum ContractType {
    GuaranteedReturn15,
    TodaysPriceGuaranteed,
    TodaysPriceGuaranteed50,
    OfCounterpartyReturn100,
    OfCounterpartyReturn75,
    OfCounterpartyReturn35,
    #[default]
    Unknown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyContracts(pub Vec<MintyContract>);

#[derive(Debug, Clone)]
pub struct SetContractEvent(pub MintyContract);

impl Event for SetContractEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}