#![allow(clippy::new_ret_no_self)]

use pelican_ui::Context;
use crate::BITCOIN_PRICE;
use crate::MintyContract;
use crate::ContractType;

use pelican_ui_std::{
    DataItem,
    Button,
    ListItem,
    AvatarContent,
    AvatarIconStyle,
    NavigateEvent,
    TextInput,
    Timestamp
};

use bitcoin::{NANS, format_usd, format_nano_btc, format_address};

pub struct DataItemMinty;

impl DataItemMinty {
    pub fn confirm_prediction(ctx: &mut Context, is_risky: bool, to_edit_prediction: Option<usize>) -> DataItem {
        let edit_prediction = to_edit_prediction.map(|i| vec![
            Button::secondary(ctx, Some("edit"), "Edit Prediciton", None, move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(i)), None)
        ]);
        
        let contract = ctx.state().get::<MintyContract>().expect("No contract");
        let multiple = contract.prediction / BITCOIN_PRICE;
        let subtitle = match is_risky {
            true => {
                match contract.variant {
                    ContractType::AdditonalReturn285 => {
                        format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd((multiple*0.85)*contract.deposited))
                    },
                    ContractType::AdditonalReturn270 => {
                        format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd((multiple*0.7)*contract.deposited))
                    },
                    ContractType::AdditonalReturn30 => {
                        format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd((multiple*0.3)*contract.deposited))
                    },
                    _ => "".to_string(),
                }
            },
            false => {
                match contract.variant {
                    ContractType::GuaranteedReturn15 => {
                        format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd(contract.deposited*2.0))
                    },
                    ContractType::TodaysPriceGuaranteed => {
                        format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd((multiple*0.3)*contract.deposited))
                    },
                    ContractType::TodaysPriceGuaranteed50 => {
                        format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd((multiple*0.7)*contract.deposited))
                    },
                    _ => "".to_string(),
                }
            }
        };

        DataItem::new(ctx, None, "Expected outcome", Some(&subtitle), None, None, edit_prediction)
    }

    pub fn contract_terms(ctx: &mut Context, is_risky: bool, to_edit: Option<(usize, usize)>) -> DataItem {
        let edits = to_edit.map(|(a, b)| vec![
            Button::secondary(ctx, Some("edit"), "Edit Withdraw", None, move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(a)), None),
            Button::secondary(ctx, Some("edit"), "Edit Deposit", None, move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(b)), None)
        ]);

        let contract = ctx.state().get::<MintyContract>().expect("No contract");

        let details = match is_risky {
            true => {
                match contract.variant {
                    // ContractType::AdditonalReturn285 => {
                    //     let a = format!("Today, I will deposit {:.8} BTC", deposit / BITCOIN_PRICE);
                    //     let b = format!("In 5 years, if Bitcoin is above {}, I will withdraw {} minus {} worth of Bitcoin.", 
                    //         format_usd(BITCOIN_PRICE/3.0), format_usd(contract.prediction), format_usd(contract.prediction), 
                    //     );
                    //     format!("{a}\n\n{b}\n\n{c}")
                    // },
                    _ => "".to_string(),
                }
            },
            false => {
                match contract.variant {
                    ContractType::GuaranteedReturn15 => {
                        let a = format!("Today, I will deposit {:.8} BTC", contract.deposited / BITCOIN_PRICE);
                        let b = format!("In 5 years, if Bitcoin is below {}, my counterparty will absorb 100% of the losses, and I will withdraw {} worth of Bitcoin.", format_usd(BITCOIN_PRICE), format_usd(contract.deposited*2.0));
                        let c = format!("In 5 years, if Bitcoin is below {}, I will withdraw {:.8} BTC, but I will incure a loss.", format_usd(BITCOIN_PRICE/3.0), ((contract.deposited * 3.0) * 0.95) / BITCOIN_PRICE);
                        format!("{a}\n\n{b}\n\n{c}")
                    },
                    ContractType::TodaysPriceGuaranteed => {
                        let a = format!("Today, I will deposit {:.8} BTC", contract.deposited / BITCOIN_PRICE);
                        let b = format!("In 5 years, if Bitcoin is above {}, I will withdraw {:.8} BTC", format_usd(BITCOIN_PRICE), (contract.deposited*1.3) / BITCOIN_PRICE);
                        let c = format!("In 5 years, if Bitcoin is below {}, my counterparty will absorb 100% of the losses, and I will withdraw {} worth of Bitcoin.", format_usd(BITCOIN_PRICE), format_usd(contract.deposited));
                        let d = format!("In 5 years, if Bitcoin is below {}, I will withdraw {:.8} BTC, but I will incure a loss.", format_usd(BITCOIN_PRICE/3.0), ((contract.deposited * 3.0) * 0.95) / BITCOIN_PRICE);
                        format!("{a}\n\n{b}\n\n{c}\n\n{d}")
                    },
                    ContractType::TodaysPriceGuaranteed50 => {
                        let a = format!("Today, I will deposit {:.8} BTC", contract.deposited / BITCOIN_PRICE);
                        let b = format!("In 5 years, if Bitcoin is above {}, I will withdraw {:.8} BTC", format_usd(BITCOIN_PRICE), (contract.deposited*1.7) / BITCOIN_PRICE);
                        let c = format!("In 5 years, if Bitcoin is between {} and {}, I will get {:.8} BTC back.", format_usd(BITCOIN_PRICE), format_usd(BITCOIN_PRICE/2.0), contract.deposited / BITCOIN_PRICE);
                        let d = format!("In 5 years, if Bitcoin is below {}, I will withdraw {:.8} BTC, but I will incure a loss.", format_usd(BITCOIN_PRICE/3.0), ((contract.deposited * 3.0) * 0.95) / BITCOIN_PRICE);
                        format!("{a}\n\n{b}\n\n{c}\n\n{d}")
                    },
                    _ => "".to_string(),
                }
            }
        };
        
        DataItem::new(ctx, None, "Contract terms", Some(&details), None, None, edits)
    }

    pub fn contract_details(ctx: &mut Context, contract: &MintyContract) -> DataItem {
        let timestamp = Timestamp::new(contract.timestamp.into());
        let (date, time) = (timestamp.date(), timestamp.time());
        // let nano_btc = &format_nano_btc(btc * NANS);
        // let usd = &format_usd(btc*price);
        let price = format_usd(BITCOIN_PRICE);
        let address = format_address(contract.address.clone());
        let withdraw = (contract.expected_amt / contract.prediction)*NANS;

        let deposit = format_nano_btc((contract.deposited / BITCOIN_PRICE)*NANS);
        let withdraw = format_nano_btc(withdraw);
        let deposit_usd = format_usd(contract.deposited);

        let details: Vec<(&str, &str)> = vec![
            ("Date", &date),
            ("Time", &time),
            ("Amount deposited", &deposit_usd),
            ("Amount deposited (nb)", &deposit),
            ("Amount to withdraw (5 years)", &withdraw),
            ("Bitcoin price", &price),
            ("Redeposit address", &address),
        ];

        DataItem::new(ctx, None, "Contract details", None, None, Some(details), None)
    }
}

pub struct ListItemMinty;
impl ListItemMinty {
    pub fn new(ctx: &mut Context, prediction: f64, deposit: f64, i: usize) -> ListItem {
        let prediction = format_usd(prediction);
        let deposit = format!("Deposit required: {}", format_usd(deposit));

        ListItem::new(
            ctx, true, &prediction, None, Some(&deposit), None, None, None, None,
            Some(AvatarContent::Icon("brand", AvatarIconStyle::Brand)), None,
            true, move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(i))
        )
    }

    pub fn contract(ctx: &mut Context, prediction: f64, pending: bool, next: impl FnMut(&mut Context) + 'static) -> ListItem {
        let prediction = format_usd(prediction);
        // let deposit = format!("Deposit required: {}", format_usd(deposit));
        let color = ctx.theme.colors.text.heading;
        let title = if pending {"Pending Contract"} else {"Open Contract"};
        let flair = pending.then_some(("warning", color));
        ListItem::new(
            ctx, true, title, flair, Some(&prediction), None, None, None, None,
            Some(AvatarContent::Icon("brand", AvatarIconStyle::Brand)), None, true, next
        )
    }
}

pub struct TextInputMinty;
impl TextInputMinty {
    pub fn address(ctx: &mut Context) -> TextInput {
        TextInput::new(
            ctx, None, Some("Bitcoin address"), "Bitcoin address...", 
            Some("Bitcoin will be redeposited here if your offer is not accepted within 30 days (8/16/25)"), 
            TextInput::NO_ICON, false
        )
    }
}