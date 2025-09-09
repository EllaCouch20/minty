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

        let contract = ctx.state().get_or_default::<MintyContract>().clone();
        let cvals = MintyContractValues::get(ctx, contract.variant);
        
        let subtitle = match is_risky {
            true => {
                // match contract.variant {
                //     ContractType::AdditonalReturn285 => {
                //         format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd(add_return_withdraw))
                //     },
                //     ContractType::AdditonalReturn270 => {
                //         format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd(add_return_withdraw))
                //     },
                //     ContractType::AdditonalReturn30 => {
                //         format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd(add_return_withdraw))
                //     },
                //     _ => "".to_string(),
                // }
                format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd(cvals.add_return_withdraw))
            },
            false => {
                // match contract.variant {
                //     ContractType::GuaranteedReturn15 => {
                //         format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd(contract.deposited*2.0))
                //     },
                //     ContractType::TodaysPriceGuaranteed => {
                //         format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd((multiple*0.3)*contract.deposited))
                //     },
                //     ContractType::TodaysPriceGuaranteed50 => {
                //         format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd((multiple*0.7)*contract.deposited))
                //     },
                //     _ => "".to_string(),
                // }
                format!("I will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd(cvals.reduced_risk_withdraw))
            }
        };

        DataItem::new(ctx, None, "Expected outcome", Some(&subtitle), None, None, edit_prediction)
    }

    pub fn contract_terms(ctx: &mut Context, is_risky: bool, to_edit: Option<(usize, usize)>) -> DataItem {
        let edits = to_edit.map(|(a, b)| vec![
            Button::secondary(ctx, Some("edit"), "Edit Withdraw", None, move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(a)), None),
            Button::secondary(ctx, Some("edit"), "Edit Deposit", None, move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(b)), None)
        ]);

        
        let contract = ctx.state().get_or_default::<MintyContract>().clone();
        let cvals = MintyContractValues::get(ctx, contract.variant);

        let details = match is_risky {
            true => {
                match contract.variant {
                    ContractType::OfCounterpartyReturn100 => {
                        let a = format!("Today, I will deposit {:.8} BTC", contract.deposited / BITCOIN_PRICE);
                        let b = format!("In 5 years, if Bitcoin is above {} I will withdraw {} and 100% of the counterparty's price appreciation after 15% per year.", 
                            format_usd(BITCOIN_PRICE*2.0), cvals.contract_after_fees / BITCOIN_PRICE 
                        );
                        let c = format!("In 5 years, if Bitcoin is below {} my counterparty will withdraw {} from the {:.8} BTC and I will withdraw the remaining Bitcoin.", 
                            format_usd(BITCOIN_PRICE*2.0), format_usd(contract.deposited * 2.0), cvals.contract_after_fees / BITCOIN_PRICE
                        );
                        let d = format!("In 5 years, if Bitcoin is below {} I will withdraw nothing.", format_usd(cvals.collateral_runs_out));
                        format!("{a}\n\n{b}\n\n{c}\n\n{d}")
                    },
                    ContractType::OfCounterpartyReturn75 => {
                        let a = format!("Today, I will deposit {:.8} BTC", contract.deposited / BITCOIN_PRICE);
                        let b = format!("In 5 years, if Bitcoin is above {} I will withdraw {:.8} BTC and 75% of the counterparty's price appreciation.", 
                            format_usd(BITCOIN_PRICE), cvals.contract_after_fees / BITCOIN_PRICE
                        );
                        let c = format!("In 5 years, if Bitcoin is below {} my counterparty will withdraw {} from the {:.8} BTC and I will withdraw the remaining Bitcoin.", 
                            format_usd(BITCOIN_PRICE), format_usd(BITCOIN_PRICE), cvals.contract_after_fees / BITCOIN_PRICE
                        );
                        format!("{a}\n\n{b}\n\n{c}")
                    },
                    ContractType::OfCounterpartyReturn35 => {
                        let a = format!("Today, I will deposit {:.8} BTC", contract.deposited / BITCOIN_PRICE);
                        let b = format!("In 5 years, if Bitcoin is above {:.8} I will withdraw {}.", 
                            format_usd(BITCOIN_PRICE), cvals.contract_after_fees / BITCOIN_PRICE
                        );
                        let c = format!("In 5 years, if Bitcoin is between {} and {}, I will withdraw {:.8}.", 
                            format_usd(BITCOIN_PRICE/2.0), format_usd(BITCOIN_PRICE), cvals.contract_after_fees / BITCOIN_PRICE
                        );
                        let d = format!("In 5 years, if Bitcoin is below {}, my counterparty will withdraw {} from the {:.8} and I will withdraw the remaining Bitcoin.", 
                            format_usd(BITCOIN_PRICE/2.0), format_usd(BITCOIN_PRICE/2.0), cvals.contract_after_fees / BITCOIN_PRICE
                        );
                        format!("{a}\n\n{b}\n\n{c}")
                    },
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MintyContractValues {
    pub reduced_risk_deposit: f64,
    pub guaranteed_withdraw: f64,
    pub total_deposit: f64,
    pub fee: f64,
    pub contract_after_fees: f64,
    pub rr_investment_start: Option<f64>,
    pub rr_investment_end: Option<f64>,
    pub rr_return: Option<f64>,
    pub collateral_runs_out: f64,
    pub reduced_risk_withdraw: f64,
    pub add_return_withdraw: f64
}

impl MintyContractValues {
    pub fn get(ctx: &mut Context, variant: ContractType) -> Self {
        let contract = ctx.state().get::<MintyContract>().expect("No contract");
        let return_percent = 0.25;

        match variant {
            ContractType::OfCounterpartyReturn100 => {
                let reduced_risk_deposit = contract.deposited / 2.0;
                println!("reduced_risk_deposit: {:?}", reduced_risk_deposit);

                let guaranteed_withdraw = reduced_risk_deposit * 2.0;
                println!("guaranteed_withdraw: {:?}", guaranteed_withdraw);

                let total_deposit = contract.deposited + reduced_risk_deposit;
                println!("total_deposit: {:?}", total_deposit);

                let fee = total_deposit * 0.05;
                println!("fee: {:?}", fee);

                let contract_after_fees = total_deposit - fee;
                println!("contract_after_fees: {:?}", contract_after_fees);

                let collateral_runs_out = guaranteed_withdraw / contract_after_fees;
                println!("collateral_runs_out: {:?}", collateral_runs_out);

                let reduced_risk_withdraw = if contract.prediction > guaranteed_withdraw {
                    guaranteed_withdraw
                } else {
                    contract_after_fees * contract.prediction
                };
                println!("reduced_risk_withdraw: {:?}", reduced_risk_withdraw);

                let add_return_withdraw = contract_after_fees - reduced_risk_withdraw;
                println!("add_return_withdraw (CALCULATED TOTAL): {:?}", add_return_withdraw);

                MintyContractValues { 
                    reduced_risk_deposit, guaranteed_withdraw, total_deposit, fee, contract_after_fees, 
                    rr_investment_start: None, rr_investment_end: None, rr_return: None, collateral_runs_out, 
                    reduced_risk_withdraw, add_return_withdraw
                }
            }

            ContractType::OfCounterpartyReturn75 => {
                let reduced_risk_deposit = contract.deposited / 2.0;
                let guaranteed_withdraw = reduced_risk_deposit;
                let total_deposit = contract.deposited + reduced_risk_deposit;
                let fee = total_deposit * 0.05;
                let contract_after_fees = total_deposit - fee;
                let return_percent = 0.25;
                let rr_investment_start = Some(reduced_risk_deposit);
                let rr_investment_end = Some(rr_investment_start.unwrap() * contract.prediction);
                let rr_return = Some((rr_investment_end.unwrap() - rr_investment_start.unwrap()) * return_percent);
                let collateral_runs_out = guaranteed_withdraw / contract_after_fees;
                let reduced_risk_withdraw = if contract.prediction > BITCOIN_PRICE {rr_return.unwrap()+reduced_risk_deposit} else {contract_after_fees*contract.prediction};
                let add_return_withdraw = contract_after_fees - reduced_risk_withdraw;
                MintyContractValues { reduced_risk_deposit, guaranteed_withdraw, total_deposit, fee, contract_after_fees, rr_investment_start, rr_investment_end, rr_return, collateral_runs_out, reduced_risk_withdraw, add_return_withdraw}
            },
            ContractType::OfCounterpartyReturn35 => {
                let reduced_risk_deposit = contract.deposited / 2.0;
                let guaranteed_withdraw = reduced_risk_deposit / 2.0;
                let total_deposit = contract.deposited + reduced_risk_deposit;
                let fee = total_deposit * 0.05;
                let contract_after_fees = total_deposit - fee;
                let return_percent = 0.25;
                let rr_investment_start = Some(reduced_risk_deposit);
                let rr_investment_end = Some(rr_investment_start.unwrap() * contract.prediction);
                let rr_return = Some((rr_investment_end.unwrap() - rr_investment_start.unwrap()) * return_percent);
                let collateral_runs_out = guaranteed_withdraw / contract_after_fees;
                let reduced_risk_withdraw = if contract.prediction > BITCOIN_PRICE / 2.0 {rr_return.unwrap()+reduced_risk_deposit} else {contract_after_fees*contract.prediction};
                let add_return_withdraw = contract_after_fees - reduced_risk_withdraw;
                MintyContractValues { reduced_risk_deposit, guaranteed_withdraw, total_deposit, fee, contract_after_fees, rr_investment_start, rr_investment_end, rr_return, collateral_runs_out, reduced_risk_withdraw, add_return_withdraw}
            },
            _ => MintyContractValues::default()
        }
    }

        // // 100% of counterparty's return
        // let reduced_risk_deposit = contract.deposited / 2.0;
        // let guaranteed_withdraw = reduced_risk_deposit * 2.0;
        // let total_deposit = contract.deposited + reduced_risk_deposit;
        // let fee = total_deposit * 0.05;
        // let contract_after_fees = total_deposit - fee;
        // let collateral_runs_out = guaranteed_withdraw / contract_after_fees;
        // let reduced_risk_withdraw = if collateral_runs_out {contract_after_fees * contract.prediction} else {guaranteed_withdraw};
        // let add_return_withdraw = contract_after_fees - reduced_risk_withdraw;

        // // 75% of counterparty's return
        // let reduced_risk_deposit = contract.deposited / 2.0;
        // let guaranteed_withdraw = reduced_risk_deposit;
        // let total_deposit = contract.deposited + reduced_risk_deposit;
        // let fee = total_deposit * 0.05;
        // let contract_after_fees = total_deposit - fee;
        // let return_percent = 0.25;
        // let rr_investment_start = reduced_risk_deposit;
        // let rr_investment_end = rr_investment_start * contract.prediction;
        // let rr_return = (rr_investment_end - rr_investment_start) * return_percent;
        // let collateral_runs_out = guaranteed_withdraw / contract_after_fees;
        // let reduced_risk_withdraw = if BITCOIN_PRICE > contract.prediction {rr_return+reduced_risk_deposit} else {contract_after_fees*contract.prediction};
        // let add_return_withdraw = contract_after_fees - reduced_risk_withdraw;

        // // 35% of counterparty's return
        // let reduced_risk_deposit = contract.deposited / 2.0;
        // let guaranteed_withdraw = reduced_risk_deposit / 2.0;
        // let total_deposit = contract.deposited + reduced_risk_deposit;
        // let fee = total_deposit * 0.05;
        // let contract_after_fees = total_deposit - fee;
        // let return_percent = 0.25;
        // let rr_investment_start = reduced_risk_deposit;
        // let rr_investment_end = rr_investment_start * contract.prediction;
        // let rr_return = (rr_investment_end - rr_investment_start) * return_percent;
        // let collateral_runs_out = guaranteed_withdraw / contract_after_fees;
        // let reduced_risk_withdraw = if contract.prediction > BITCOIN_PRICE / 2.0 {rr_return+reduced_risk_deposit} else {contract_after_fees*contract.prediction};
        // let add_return_withdraw = contract_after_fees - reduced_risk_withdraw;
}