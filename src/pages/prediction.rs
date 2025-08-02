use pelican_ui::events::{OnEvent, Event, TickEvent};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Header,
    Content, Bumper,
    NavigateEvent, Button,
    Offset, AppPage,
    ButtonState, IconButton,
    IS_MOBILE
};

use crate::{ MintyContract, BitcoinDeposit, MintyHome };
use bitcoin::components::{AmountInput, NumericKeypad};

#[derive(Debug, Component)]
pub struct BitcoinPrediction(Stack, Page, #[skip] ButtonState);
impl AppPage for BitcoinPrediction {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(MintyHome::new(ctx))),
            1 => Ok(Box::new(BitcoinDeposit::new(ctx, false))),
            2 => Ok(Box::new(BitcoinDeposit::new(ctx, true))),
            _ => Err(self)
        }
    }
}

impl BitcoinPrediction {
    pub fn new(ctx: &mut Context) -> Self {
        let msg = if IS_MOBILE {"Enter expected price of bitcoin in 5 years."} else {"Type expected price of bitcoin in 5 years."};
        

        let mut prediction = ctx.state().get_mut::<MintyContract>().map(|c| c.prediction).unwrap_or(0.0);
        if prediction == 0.0 {prediction = 500_000.0;} //else {prediction = (prediction/NANS)*BITCOIN_PRICE;}
        let mut amount_display = AmountInput::new(ctx, Some((prediction, msg)), false);
        amount_display.set_max(f64::MAX);

        let numeric_keypad = NumericKeypad::new(ctx);
        let mut content: Vec<Box<dyn Drawable>> = vec![Box::new(amount_display)];
        IS_MOBILE.then(|| content.push(Box::new(numeric_keypad)));
        let content = Content::new(Offset::Center, content);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Bitcoin prediction", None);

        let reduced = Button::primary(ctx, "Reduce Risk", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let added = Button::primary(ctx, "Add Risk", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(2)));
        let bumper = Bumper::double_button(ctx, reduced, added);
        BitcoinPrediction(Stack::default(), Page::new(Some(header), content, Some(bumper)), ButtonState::Default)
    }
}

impl OnEvent for BitcoinPrediction {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let amount = &mut *self.1.content().find::<AmountInput>().unwrap();
            // ctx.state().set(&SendAmount::new(*amount.btc() as f64));
            let usd = amount.usd().trim().replace(',', "").parse::<f64>().unwrap();
            // let usd = usd.parse::<f64>().expect(&format!("Could not parse {:?}", usd));
            ctx.state().get_mut::<MintyContract>().unwrap().prediction = usd; //(*amount.btc()*BITCOIN_PRICE)/NANS; // usd amount
            // println!("Set to {}", ctx.state().get_mut::<MintyContract>().unwrap().prediction);
            let error = *amount.error();
            let button = &mut self.1.bumper().as_mut().unwrap().find_at::<Button>(0).unwrap();
            button.update_state(ctx, error, !error, &mut self.2);
            let button = &mut self.1.bumper().as_mut().unwrap().find_at::<Button>(1).unwrap();
            button.update_state(ctx, error, !error, &mut self.2);
        }
        true
    }
}