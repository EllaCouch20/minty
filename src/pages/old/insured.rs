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
pub struct InsuredPrice(Stack, Page, #[skip] ButtonState);

impl AppPage for InsuredPrice {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(ExpectedAmount::new(ctx, false))),
            1 => Ok(Box::new(ConfirmContract::new(ctx, false))),
            _ => Err(self)
        }
    }
}

impl InsuredPrice {
    pub fn new(ctx: &mut Context) -> Self {
        // let msg = if IS_MOBILE {"You are protected from BTC price dips unless it drops below this number."} else {"Type the amount of bitcoin you deposit."};

        // let msg = "Enter the minimum amount of bitcoin you will withdraw in 5 years.";
        let msg = "Enter the lowest price you believe Bitcoin could be in 5 years.";

        let mut minimum = ctx.state().get_mut::<MintyContract>().map(|c| c.minimum).unwrap_or(0.0);
        // if minimum == 0.0 {minimum = BITCOIN_PRICE;} //else {minimum = (minimum/NANS)*BITCOIN_PRICE;}
        let mut amount_display = AmountInput::new(ctx, Some((minimum, msg)), false);
        amount_display.set_max(f64::MAX);
        
        let numeric_keypad = NumericKeypad::new(ctx);
        let mut content: Vec<Box<dyn Drawable>> = vec![Box::new(amount_display)];
        IS_MOBILE.then(|| content.push(Box::new(numeric_keypad)));
        let content = Content::new(Offset::Center, content);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Insured price", None);

        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        // let add_risk = Button::primary(ctx, "Add Risk", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let bumper = Bumper::single_button(ctx, button);
        InsuredPrice(Stack::default(), Page::new(Some(header), content, Some(bumper)), ButtonState::Default)
    }
}

impl OnEvent for InsuredPrice {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let amount = &mut *self.1.content().find::<AmountInput>().unwrap();
            // ctx.state().set(&SendAmount::new(*amount.btc() as f64));
            let mut usd = amount.usd().trim().replace(',', "").parse::<f64>().unwrap();
            ctx.state().get_mut::<MintyContract>().unwrap().minimum = usd; //*amount.btc();
            // println!("Set to {}", ctx.state().get_mut::<MintyContract>().unwrap().deposit);
            let error = *amount.error();
            let button = &mut self.1.bumper().as_mut().unwrap().find::<Button>().unwrap();
            button.update_state(ctx, error, !error, &mut self.2);
        }
        true
    }
}
