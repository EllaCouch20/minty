use pelican_ui::events::{OnEvent, Event, TickEvent};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Button, ButtonState,
    Page, Stack, Bumper,
    NavigateEvent, Header,
    IconButton, Offset,
    Content, IS_MOBILE,
    AppPage,
};

use crate::{
    MintyContract, 
    SelectContract,
    MintyHome, 
};

use bitcoin::components::{AmountInput, NumericKeypad};

#[derive(Debug, Component)]
pub struct BitcoinDeposit(Stack, Page, #[skip] bool, #[skip] ButtonState);

impl AppPage for BitcoinDeposit {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(MintyHome::new(ctx))),
            1 => Ok(Box::new(SelectContract::new(ctx, self.2))),
            _ => Err(self)
        }
    }
}

impl BitcoinDeposit {
    pub fn new(ctx: &mut Context, is_risky: bool) -> Self {
        let msg = if IS_MOBILE {"Enter amount of bitcoin you will deposit."} else {"Type the amount of bitcoin you will deposit."};

        let deposit = ctx.state().get_mut::<MintyContract>().map(|c| c.deposited).unwrap_or(0.0);
        let mut amount_display = AmountInput::new(ctx, Some((deposit, msg)), false);
        amount_display.set_max(f64::MAX);

        let numeric_keypad = NumericKeypad::new(ctx);
        let mut content: Vec<Box<dyn Drawable>> = vec![Box::new(amount_display)];
        IS_MOBILE.then(|| content.push(Box::new(numeric_keypad)));
        let content = Content::new(Offset::Center, content);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Bitcoin deposit", None);

        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        // let add_risk = Button::primary(ctx, "Add Risk", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let bumper = Bumper::single_button(ctx, button);
        BitcoinDeposit(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky, ButtonState::Default)
    }
}

impl OnEvent for BitcoinDeposit {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let amount = &mut *self.1.content().find::<AmountInput>().unwrap();
            let usd = amount.usd().trim().replace(',', "").parse::<f64>().unwrap();
            // ctx.state().set(&SendAmount::new(*amount.btc() as f64));
            ctx.state().get_mut_or_default::<MintyContract>().deposited = usd;
            // println!("Set to {}", ctx.state().get_mut::<MintyContract>().unwrap().deposit);
            let error = *amount.error();
            let button = &mut self.1.bumper().as_mut().unwrap().find::<Button>().unwrap();
            button.update_state(ctx, error, !error, &mut self.3);
        }
        true
    }
}