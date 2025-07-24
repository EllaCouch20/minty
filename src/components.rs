#![allow(clippy::new_ret_no_self)]

use pelican_ui::Context;
use crate::BITCOIN_PRICE;
use crate::MintyContract;

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

use bitcoin::{NANS, format_usd, format_nano_btc};

pub struct DataItemMinty;

impl DataItemMinty {
    pub fn confirm_prediction(ctx: &mut Context, _prediction: f64, _dip: f64, 
        to_edit_prediction: impl FnMut(&mut Context) + 'static,
    ) -> DataItem {
        let edit_prediction = Button::secondary(ctx, Some("edit"), "Edit Prediciton", None, to_edit_prediction, None);
        let contract = ctx.state().get::<MintyContract>().expect("No contract");
        let subtitle = format!("You will deposit {} now and withdraw {} in 5 years.", format_usd(contract.deposited), format_usd(contract.expected_amt));
        DataItem::new(ctx, None, "Expected outcome", Some(&subtitle), None, None, Some(vec![edit_prediction]))
    }

    pub fn contract_terms(ctx: &mut Context, is_risky: bool,
        to_edit_protection: impl FnMut(&mut Context) + 'static,
        to_edit_deposit: impl FnMut(&mut Context) + 'static,
    ) -> DataItem {
        let edit_protection = Button::secondary(ctx, Some("edit"), "Edit Withdraw", None, to_edit_protection, None);
        let edit_deposit = Button::secondary(ctx, Some("edit"), "Edit Deposit", None, to_edit_deposit, None);

        let contract = ctx.state().get::<MintyContract>().expect("No contract");

        let deposit = contract.deposited;//12_478.00;
        let usd_prediction = contract.prediction; //118_002.44;
        let btc_minimum = contract.minimum; //20_384.19; // insured price
        // let withdraw = contract.withdraw; //20_383.18;
        // let usd_deposit = (contract.deposited/NANS)*BITCOIN_PRICE;

        let withdraw = (contract.expected_amt / usd_prediction)*NANS; // risky

        let fmt_deposit = format_nano_btc((deposit / BITCOIN_PRICE)*NANS);
        // let fmt_prediction = format_usd(_prediction);
        let fmt_minimum = format_usd(btc_minimum);
        let fmt_withdraw = format_nano_btc(withdraw);
        // let fmt_usd_deposit = format_usd(usd_deposit);

        let percentage = 20;

        let details = if is_risky {
            let a = format!("Today, I will deposit {}.", fmt_deposit);
            let b = format!("In 5 years, if Bitcoin is above {}, I will withdraw {}.", format_usd(BITCOIN_PRICE), fmt_withdraw);
            // let c = format!("In 5 years, if Bitcoin is below {}, I will absorb 100% of the counterparty losses.", fmt_prediction);
            let c = format!("In 5 years, if Bitcoin is below {}, I will lose my deposit.", format_usd(BITCOIN_PRICE));
            format!("{}\n\n{}\n\n{}", a, b, c)
        } else {
            let a = format!("Today, I will deposit {}.", fmt_deposit);
            let b = format!("In 5 years, if Bitcoin is above {}, I will withdraw {}.", format_usd(BITCOIN_PRICE), fmt_withdraw);
            let c = format!("In 5 years, if Bitcoin is below {}, my counterparty will absorb 100% of the losses, and I will withdraw {} worth of Bitcoin.", format_usd(BITCOIN_PRICE), format_usd(contract.deposited));
            let d = format!("In 5 years, if Bitcoin is below {}, I will lose my deposit.", fmt_minimum);
            format!("{}\n\n{}\n\n{}\n\n{}", a, b, c, d)
        };

        // let subtitle = format!("{}\n\n{}\n\n{}\n\n{}", details.0, details.1, details.2, details.3);
        DataItem::new(ctx, None, "Contract terms", Some(&details), None, None, Some(vec![edit_protection, edit_deposit]))
    }

    pub fn view_prediction(ctx: &mut Context, _prediction: f64, _dip: f64) -> DataItem {
        let subtitle = format!("You will get {}", format_usd(500000.00));
        let desc = format!("Assuming the bitcoin price is {} in 5 years.", format_usd(200000.00));

        DataItem::new(ctx, None, "Expected withdraw", Some(&subtitle), Some(&desc), None, None)
    }

    pub fn view_deposit(
        ctx: &mut Context, price: f64, deposit: f64, _prediction: f64,
    ) -> DataItem {
        let deposit_usd = &format_usd(price*deposit);
        let deposit_nano = &format_nano_btc(deposit*NANS);

        let details: Vec<(&str, &str)> = vec![
            ("Amount to deposit", deposit_usd),
            ("Amount to deposit (nb)", deposit_nano),
        ];

        DataItem::new(ctx, None, "Bitcoin deposit", None, None, Some(details), None)
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
            move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(i))
        )
    }

    pub fn contract(ctx: &mut Context, prediction: f64, date: Timestamp, i: usize) -> ListItem {
        let prediction = format_usd(prediction);
        // let deposit = format!("Deposit required: {}", format_usd(deposit));
        ListItem::new(
            ctx, true, "Minty Contract", None, Some(&date.friendly()), 
            None, Some(&prediction), Some("Details"), None,
            Some(AvatarContent::Icon("brand", AvatarIconStyle::Brand)), None,
            move |ctx: &mut Context| ctx.trigger_event(NavigateEvent(i))
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