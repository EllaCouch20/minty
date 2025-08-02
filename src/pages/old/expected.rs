use pelican_ui::events::{OnEvent, Event, TickEvent};
use pelican_ui::drawable::{Align, Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Stack, Content, Header, 
    Bumper, Page, Button, 
    Offset, TextStyle,
    Brand, NavigateEvent, 
    AppPage, ExpandableText,
    IconButton, ButtonState, 
    IS_MOBILE, ListItemGroup,
    AvatarContent, AvatarIconStyle,
    Avatar, TextInput, SetActiveInput,
    QuickActions, QRCode, Timestamp,
    ListItemSelector,
};

use bitcoin::{format_usd, NANS};
use bitcoin::components::{AmountInput, NumericKeypad, AmountDisplay};
use crate::{ContractType, SetContractEvent, MintyPlugin, DataItemMinty, ListItemMinty, TextInputMinty, MyContracts, MintyContract};
use crate::service::MintyRequest;
use chrono::Local;

#[derive(Debug, Component)]
pub struct ExpectedAmount(Stack, Page, #[skip] bool, #[skip] ButtonState);

impl AppPage for ExpectedAmount {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(BitcoinDeposit::new(ctx, self.2))),
            1 if self.2 => Ok(Box::new(ConfirmContract::new(ctx, true))),
            1 => Ok(Box::new(InsuredPrice::new(ctx))),
            _ => Err(self)
        }
    }
}

impl ExpectedAmount {
    pub fn new(ctx: &mut Context, is_risky: bool) -> Self {
        // let msg = if IS_MOBILE {"You are protected from BTC price dips unless it drops below this number."} else {"Type the amount of bitcoin you deposit."};

        let msg = "Enter the amount of bitcoin you will withdraw if your price prediction is correct.";

        let mut expected_amt = ctx.state().get_mut::<MintyContract>().map(|c| c.expected_amt).unwrap_or(0.0);
        // if expected_amt == 0.0 {expected_amt = BITCOIN_PRICE;} //else {expected_amt = (expected_amt/NANS)*BITCOIN_PRICE;}
        let mut amount_display = AmountInput::new(ctx, Some((expected_amt, msg)), false);
        amount_display.set_max(f64::MAX);

        let numeric_keypad = NumericKeypad::new(ctx);
        let mut content: Vec<Box<dyn Drawable>> = vec![Box::new(amount_display)];
        IS_MOBILE.then(|| content.push(Box::new(numeric_keypad)));
        let content = Content::new(Offset::Center, content);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Expected amount", None);

        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        // let add_risk = Button::primary(ctx, "Add Risk", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let bumper = Bumper::single_button(ctx, button);
        ExpectedAmount(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky, ButtonState::Default)
    }
}

impl OnEvent for ExpectedAmount {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let amount = &mut *self.1.content().find::<AmountInput>().unwrap();
            // ctx.state().set(&SendAmount::new(*amount.btc() as f64));
            let mut usd = amount.usd().trim().replace(',', "").parse::<f64>().unwrap();
            ctx.state().get_mut::<MintyContract>().unwrap().expected_amt = usd;//*amount.btc();
            // println!("Set to {}", ctx.state().get_mut::<MintyContract>().unwrap().deposit);
            let error = *amount.error();
            let button = &mut self.1.bumper().as_mut().unwrap().find::<Button>().unwrap();
            button.update_state(ctx, error, !error, &mut self.3);
        }
        true
    }
}