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

pub const BITCOIN_PRICE: f64 = 118_000.00;

#[derive(Debug, Component)]
pub struct MintyTerms(Stack, Page);

impl OnEvent for MintyTerms {}

impl AppPage for MintyTerms {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(MintyHome::new(ctx))),
            _ => Err(self)
        }
        // Ok(self)
    }
}

impl MintyTerms {
    pub fn new(ctx: &mut Context) -> Self {
        let text_size = ctx.theme.fonts.size.lg;

        let bullet1 = ExpandableText::new(ctx, "Minty is an open source browser for smart contracts on the Liquid blockchain.", TextStyle::Primary, text_size, Align::Center, None);
        let bullet2 = ExpandableText::new(ctx, "Minty uses your expected bitcoin price to make the contract terms easier to view. Be sure to update it.", TextStyle::Primary, text_size, Align::Center, None);
        let bullet3 = ExpandableText::new(ctx, "Minty never has control of your money - you’ll use bitcoin wallet to make deposits and receive payments from Liquid.", TextStyle::Primary, text_size, Align::Center, None);
        let bullet4 = ExpandableText::new(ctx, "Anyone can make or accept a contract on Liquid and the terms and payments are enforced by Liquid.", TextStyle::Primary, text_size, Align::Center, None);

        let wordmark = ctx.theme.brand.wordmark.clone();
        let wordmark = Brand::new(wordmark, (200.0, 50.0));
       
        let content = Content::new(Offset::Start, vec![Box::new(wordmark), Box::new(bullet1), Box::new(bullet2), Box::new(bullet3), Box::new(bullet4)]);

        let dismiss = Button::secondary_expand(ctx, "Hide Forever", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let bumper = Bumper::double_button(ctx, dismiss, button);
        MintyTerms(Stack::default(), Page::new(None, content, Some(bumper)))
    }
}


#[derive(Debug, Component)]
pub struct MintyHome(Stack, Page, #[skip] Option<MintyContract>);

impl OnEvent for MintyHome {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(SetContractEvent(contract)) = event.downcast_ref::<SetContractEvent>() {
            self.2 = Some(contract.clone());
            ctx.trigger_event(NavigateEvent(0));
        }
        true
    }
}

impl AppPage for MintyHome {
    fn has_nav(&self) -> bool { true }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(ContractDetails::new(ctx, self.2.unwrap()))),
            1 => Ok(Box::new(BitcoinDeposit::new(ctx, false))),
            2 => Ok(Box::new(BitcoinDeposit::new(ctx, true))),
            _ => Err(self)
        }
    }
}

impl MintyHome {
    pub fn new(ctx: &mut Context) -> Self {
        // let text_size = ctx.theme.fonts.size.sm;

        // let text = "No existing contracts matched.\nSelect a different contract below or publish your offer.";
        // let text = ExpandableText::new(ctx, text, TextStyle::Primary, text_size, Align::Center, None);

        let price = format_usd(BITCOIN_PRICE);
        let amount = ctx.state().get::<MyContracts>().map(|con| con.0.iter().map(|c| c.expected_amt).sum::<f64>().max(0.0)).unwrap_or(0.0);
        let amount = if amount == 0.0 {"$0.00".to_string()} else {format_usd(amount)};
        let display = AmountDisplay::new(ctx, &amount, "Expected contracts value");
        let mut content: Vec<Box<dyn Drawable>> = vec![Box::new(display)];
        let mut offset = Offset::Center;

        if let Some(contracts) = ctx.state().get::<MyContracts>() {
            if contracts.0.len() >= 1 {
                let mut cs = contracts.0.clone();
                cs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
                let list_items = cs.into_iter().map(|c| {
                    ListItemMinty::contract(ctx, c.expected_amt, !c.accepted, 
                        move |ctx: &mut Context| ctx.trigger_event(SetContractEvent(c.clone()))
                    )
                }).collect();
                content.push(Box::new(ListItemGroup::new(list_items)) as Box<dyn Drawable>);
                offset = Offset::Start;
            }
        }

        // let button = Button::primary(ctx, "Make Prediction", |ctx: &mut Context| {
        //     ctx.state().set(MintyContract::empty());
        //     ctx.trigger_event(NavigateEvent(1));
        // });

        ctx.state().set(MintyContract::empty());
        let reduced = Button::primary(ctx, "Reduce Risk", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let added = Button::primary(ctx, "Add Return", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(2)));
        let bumper = Bumper::double_button(ctx, reduced, added);
        let content = Content::new(offset, content);
        let header = Header::home(ctx, "Minty", None);
        MintyHome(Stack::default(), Page::new(Some(header), content, Some(bumper)), None)
    }
}

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
            let mut usd = amount.usd().trim().replace(',', "").parse::<f64>().unwrap();
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

        let mut deposit = ctx.state().get_mut::<MintyContract>().map(|c| c.deposited).unwrap_or(0.0);
        // if deposit == 0.0 {deposit = BITCOIN_PRICE;}// else {deposit = (deposit/NANS)*BITCOIN_PRICE;}
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
            let mut usd = amount.usd().trim().replace(',', "").parse::<f64>().unwrap();
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


#[derive(Debug, Component)]
pub struct ConfirmContract(Stack, Page, #[skip] bool, #[skip] Option<MintyContract>);

impl OnEvent for ConfirmContract {}

impl AppPage for ConfirmContract {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(ExpectedAmount::new(ctx, self.2))),
            1 => Ok(Box::new(BitcoinPrediction::new(ctx))),
            2 => Ok(Box::new(BitcoinDeposit::new(ctx, self.2))),
            3 => match self.3 {
                Some(contract) => Ok(Box::new(ViewContract::new(ctx, self.2, true, contract))),
                None => Ok(Box::new(SimilarContracts::new(ctx, self.2, None))),
            }
            _ => Err(self)
        }
    }
}

impl ConfirmContract {
    pub fn new(ctx: &mut Context, is_risky: bool) -> Self {
        let confirm_prediction = DataItemMinty::confirm_prediction(
            ctx, is_risky, |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)),
        );

        let contract_terms = DataItemMinty::contract_terms(ctx, is_risky, 
            |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)),
            |ctx: &mut Context| ctx.trigger_event(NavigateEvent(2))
        );

        // for risky, the less risky side's deposit must be higher than the risky side's (expected withdrawal amount - deposit amount)
        // for risky, the less risky side's 'expected upside' must be lower than 2 - (risky's expected withdraw - risky's price prediction)
        // expected upside = your/their withraw \over\ your/their price prediction

        // for less, the minimum deposit is met. minimum deposit = (current price / insured amount) - my deposit
        // for less, their maximum upside is lower than 2 - (risky's expected withdraw - risky's price prediction)

        let mine = ctx.state().get::<MintyContract>().unwrap().clone();

        let mut match_found = None;
        ctx.state().get::<MyContracts>().unwrap().0.iter().filter(|c| !c.accepted).for_each(|theirs| {
            if is_risky {
                // We are the risky side of the offer
                let first = theirs.deposited > (mine.expected_amt - mine.deposited);
                let second = (theirs.expected_amt / theirs.prediction) < (2.0-(mine.expected_amt - mine.prediction));
                if first && second {
                    match_found = Some(theirs.clone());
                    println!("THIS SUPER RISKY OFFER JUST FOUND A CONTRACT!");
                }
            } else {
                // We are the less risky side of the offer
                let min_deposit = (BITCOIN_PRICE / mine.minimum) - mine.deposited;
                let first = theirs.deposited > min_deposit;
                let second = (theirs.expected_amt / theirs.prediction) < (2.0-(mine.expected_amt - mine.prediction));
                if first && second {
                    match_found = Some(theirs.clone());
                    println!("THIS LESS RISKY OFFER JUST FOUND A CONTRACT!");
                }
            };
        });

        // let text = ExpandableText::new(ctx, "You withdraw $385,000.00", TextStyle::Heading, text_size, Align::Center, None);
        let button = Button::primary(ctx, "Confirm", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(3)));
        let bumper = Bumper::single_button(ctx, button);
        let content = Content::new(Offset::Start, vec![Box::new(confirm_prediction), Box::new(contract_terms)]);
        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Confirm contract", None);
        ConfirmContract(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky, match_found)
    }
}

#[derive(Debug, Component)]
pub struct ViewContract(Stack, Page, #[skip] bool, #[skip] bool, #[skip] MintyContract);

impl OnEvent for ViewContract {}

impl AppPage for ViewContract {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 if self.3 => Ok(Box::new(ConfirmContract::new(ctx, self.2))),
            0 => Ok(Box::new(SimilarContracts::new(ctx, self.2, Some(self.4)))),
            1 => Ok(Box::new(RedepositAddress::new(ctx, None, self.2, false, self.3, self.4))),
            // 2 => Ok(Box::new(SimilarContracts::new(ctx, self.2))),
            _ => Err(self)
        }
    }
}

impl ViewContract {
    pub fn new(ctx: &mut Context, is_risky: bool, was_match: bool, contract: MintyContract) -> Self {
        let text_size = ctx.theme.fonts.size.md;

        let my = ctx.state().get::<MintyContract>().unwrap().clone();

        let text = ExpandableText::new(ctx, "A matching contract has been found.\nAccept the offer, or reject to see similar offers.", TextStyle::Secondary, text_size, Align::Center, None);
        let prediction = DataItemMinty::view_prediction(ctx, &my);
        let deposit = DataItemMinty::view_terms(ctx, is_risky, &my);

        let accept = Button::primary(ctx, "Accept", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let reject = Button::secondary_expand(ctx, "Reject", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0))); 
        let bumper = Bumper::double_button(ctx, reject, accept);
        let content = Content::new(Offset::Start, vec![Box::new(text), Box::new(prediction), Box::new(deposit)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Matching contract", None);
        ViewContract(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky, was_match, contract)
    }
}

#[derive(Debug, Component)]
pub struct SimilarContracts(Stack, Page, #[skip] bool, #[skip] Option<MintyContract>);

impl OnEvent for SimilarContracts {}

impl AppPage for SimilarContracts {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => match self.3 {
                Some(contract) => Ok(Box::new(ViewContract::new(ctx, self.2, false, contract))),
                None => Ok(Box::new(ConfirmContract::new(ctx, self.2)))
            },
            1 => {
                let contract = ctx.state().get::<MintyContract>().unwrap().clone();
                Ok(Box::new(RedepositAddress::new(ctx, None, self.2, true, false, contract)))
            }
            // 2 => Ok(Box::new(ViewContract::new(ctx, self.2, false, ))),
            // 2 => Ok(Box::new(RedepositAddress::new(ctx, None, self.2, false, false))),
            _ => Err(self)
        }
    }
}

impl SimilarContracts {
    pub fn new(ctx: &mut Context, is_risky: bool, rejected: Option<MintyContract>) -> Self {
        let text_size = ctx.theme.fonts.size.sm;

        let text = "No existing contracts matched.\nSelect a different contract below or publish your offer.";
        let text = ExpandableText::new(ctx, text, TextStyle::Primary, text_size, Align::Center, None);

        // let list_items = ListItemGroup::new(vec![
        //     ListItemMinty::new(ctx, 560_000.0, 100_000.00, 2),
        //     ListItemMinty::new(ctx, 450_000.0, 100_000.00, 2),
        //     ListItemMinty::new(ctx, 740_000.0, 100_000.00, 2),
        //     ListItemMinty::new(ctx, 566_000.0, 100_000.00, 2),
        // ]);


        let button = Button::primary(ctx, "Publish My Offer", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        
        let bumper = Bumper::single_button(ctx, button);
        let content = Content::new(Offset::Start, vec![Box::new(text)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Similar contracts", None);
        SimilarContracts(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky, rejected)
    }
}

#[derive(Debug, Component)]
pub struct RedepositAddress(Stack, Page, 
    #[skip] ButtonState, 
    #[skip] bool, // is risky offer
    #[skip] bool, // is my own offer
    #[skip] bool, // was a matching offer
    #[skip] MintyContract,
);

impl AppPage for RedepositAddress {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 if !self.5 => Ok(Box::new(SimilarContracts::new(ctx, self.3, None))),
            // 0 => Ok(Box::new(ConfirmContract::new(ctx, self.3))),
            0 => Ok(Box::new(ViewContract::new(ctx, self.3, self.5, self.6))),
            // 1 => Ok(Box::new(ContractAccepted::new(ctx, self.4))),
            1 => Ok(Box::new(QRCodeDeposit::new(ctx, self.3, self.4, self.5, self.6))),
            _ => Err(self)
        }
    }
}

impl RedepositAddress {
    fn new(ctx: &mut Context, _address: Option<String>, is_risky: bool, is_mine: bool, was_match: bool, contract: MintyContract) -> Self {
        let button = Button::disabled(ctx, "Continue", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let input = TextInputMinty::address(ctx);

        let paste = Button::secondary(ctx, Some("paste"), "Paste Clipboard", None, move |ctx: &mut Context| {
            let data = ctx.hardware.paste();
            ctx.trigger_event(SetActiveInput(data))
        }, Some("Pasted Clipboard".to_string()));

        let scan_qr = Button::secondary(ctx, Some("qr_code"), "Scan QR Code", None, |ctx: &mut Context| ctx.trigger_event(NavigateEvent(2)), None);
        // let contact = Button::secondary(ctx, Some("profile"), "Select Contact", None, |ctx: &mut Context| ctx.trigger_event(NavigateEvent(3)), None);

        let quick_actions = QuickActions::new(vec![paste, scan_qr]);
        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));

        let header = Header::stack(ctx, Some(back), "Send bitcoin", None);
        let bumper = Bumper::single_button(ctx, button);
        let content = Content::new(Offset::Start, vec![Box::new(input), Box::new(quick_actions)]);

        RedepositAddress(Stack::default(), Page::new(Some(header), content, Some(bumper)), ButtonState::Default, is_risky, is_mine, was_match, contract)
    }
}

impl OnEvent for RedepositAddress {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let input = &mut *self.1.content().find::<TextInput>().unwrap();
            let input_address = input.value().clone();

            if !input_address.is_empty() {
                // let (address, amount) = ("", None);// parse_btc_uri(input_address);
                // *input.value() = "address".to_string(); // PROBLEM
                // let address = SendAddress::new(address.to_string());
                // if let Some(b) = amount { ctx.state().set(&SendAmount::new(b)) }

                // match address.is_valid() {
                //     true => *input.error() = false,
                //     false => input.set_error(ctx, "Address is not valid.")
                // }

                // ctx.state().set(&address);
                ctx.state().get_mut::<MintyContract>().unwrap().address = input_address.clone();
            }

            let error = *input.error() || input_address.is_empty();
            let button = self.1.bumper().as_mut().unwrap().find::<Button>().unwrap();
            button.update_state(ctx, error, !error, &mut self.2);
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct QRCodeDeposit(Stack, Page, #[skip] bool, #[skip] bool, #[skip] bool, #[skip] MintyContract);
impl OnEvent for QRCodeDeposit {}

impl AppPage for QRCodeDeposit {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(RedepositAddress::new(ctx, None, self.2, self.3, self.4, self.5))),
            1 => Ok(Box::new(ContractAccepted::new(ctx, self.3, self.5))),
            _ => Err(self)
        }
    }
}

impl QRCodeDeposit {
    fn new(ctx: &mut Context, is_risky: bool, is_mine: bool, was_match: bool, contract: MintyContract) -> Self {
        let text_size = ctx.theme.fonts.size.md;
        let address = "Why are you even trying to deposit Bitcoin here??? Shameful.";

        let qr_code = QRCode::new(ctx, address);
        let text = ExpandableText::new(ctx, "Scan to deposit bitcoin and accept this contract.", TextStyle::Secondary, text_size, Align::Center, None);
        let content = Content::new(Offset::Center, vec![Box::new(qr_code), Box::new(text)]);

        let close = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(close), "Deposit bitcoin", None);

        let contract_c = contract.clone();
        let button = Button::primary(ctx, "Skip", move |ctx: &mut Context| {
            if is_mine {
                let mut guard = ctx.get::<MintyPlugin>();
                let (plugin, ctx) = guard.get();
                let my_contract = ctx.state().get::<MintyContract>().unwrap();
                plugin.request(MintyRequest::CreateContract(my_contract.clone()));
                ctx.trigger_event(NavigateEvent(1))
            } else {
                let contracts = ctx.state().get_mut::<MyContracts>().unwrap();
                contracts.0.iter_mut().for_each(|c| {
                    if *c == contract_c.clone() {
                        c.accepted = true;
                    }
                });
                ctx.trigger_event(NavigateEvent(1))
            }
        });
        let bumper = Bumper::single_button(ctx, button);
        QRCodeDeposit(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky, is_mine, was_match, contract)
    }
}


#[derive(Debug, Component)]
pub struct ContractAccepted(Stack, Page);

impl OnEvent for ContractAccepted {}

impl AppPage for ContractAccepted {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(MintyHome::new(ctx))),
            _ => Err(self)
        }
    }
}

impl ContractAccepted {
    pub fn new(ctx: &mut Context, is_mine: bool, contract: MintyContract) -> Self {

        let text_size = ctx.theme.fonts.size.h4;
        let icon = Avatar::new(ctx, AvatarContent::Icon("brand", AvatarIconStyle::Brand), None, false, 72.0, None);
        
        let (subtext, title) = match is_mine {
            true => ("Your contract has been published".to_string(), "Contract published"),
            false => (format!("You accepted a contract for {}", &format_usd(contract.deposited)), "Contract accepted"),
        };

        let text = ExpandableText::new(ctx, &subtext, TextStyle::Heading, text_size, Align::Center, None);
        let done = Button::secondary_expand(ctx, "Done", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        
        let bumper = Bumper::single_button(ctx, done);
        let content = Content::new(Offset::Center, vec![Box::new(icon), Box::new(text)]);

        let close = IconButton::close(ctx, |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(close), title, None);
        ContractAccepted(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}

#[derive(Debug, Component)]
pub struct ContractDetails(Stack, Page);

impl OnEvent for ContractDetails {}

impl AppPage for ContractDetails {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(MintyHome::new(ctx))),
            _ => Err(self)
        }
    }
}

impl ContractDetails {
    pub fn new(ctx: &mut Context, contract: MintyContract) -> Self {
        let details = DataItemMinty::contract_details(ctx, &contract);

        let done = Button::secondary_expand(ctx, "Done", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0))); 
        let bumper = Bumper::single_button(ctx, done);
        let content = Content::new(Offset::Start, vec![Box::new(details)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Contract details", None);
        ContractDetails(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}


#[derive(Debug, Component)]
pub struct SelectContract(Stack, Page, #[skip] bool);

impl AppPage for SelectContract {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(MintyHome::new(ctx))),
            1 => Ok(Box::new(ConfirmContract::new(ctx, self.2))),
            _ => Err(self)
        }
    }
}

impl SelectContract {
    fn new(ctx: &mut Context, is_risky: bool) -> Self {
        let prediction = ctx.state().get_or_default::<MintyContract>().prediction;
        let deposit = ctx.state().get_or_default::<MintyContract>().deposited;
        let text_size = ctx.theme.fonts.size.h4;
        let multiple = prediction / BITCOIN_PRICE;
        let text = ExpandableText::new(ctx, &format!("I will deposit {}", format_usd(deposit)), TextStyle::Heading, text_size, Align::Center, None);
        
        let selector = match is_risky {
            true => ListItemSelector::new(ctx,
                ("285% Additonal Return", &format!("If Bitcoin is worth {} in 5 years, I will withdraw {}.", format_usd(prediction), format_usd((multiple*0.85)*deposit)), Some(&format!("If Bitcoin is below {}, I will absorb the loss so the counterparty can withdraw at a price of {}", format_usd(BITCOIN_PRICE*2.0), format_usd(BITCOIN_PRICE*2.0)))),
                ("270% Additonal Return", &format!("If Bitcoin is worth {} in 5 years, I will withdraw {}.", format_usd(prediction), format_usd((multiple*0.7)*deposit)), Some(&format!("If Bitcoin is below {}, I will absorb the loss so the counterparty can withdraw at a price of {}", format_usd(BITCOIN_PRICE), format_usd(BITCOIN_PRICE)))),
                Some(("30% Additonal Return", &format!("If Bitcoin is worth {} in 5 years, I will withdraw {}.", format_usd(prediction), format_usd((multiple*0.3)*deposit)), Some(&format!("If Bitcoin is below {}, I will absorb the loss so the counterparty can withdraw at a price of {}", format_usd(BITCOIN_PRICE/2.0), format_usd(BITCOIN_PRICE/2.0))))),
                // ("Today's Price Guaranteed", &format!("If Bitcoin is {}, I will withdraw {}\nIf Bitcoin is below {}, I will withdraw {}", format_usd(prediction), format_usd((multiple*0.3)*deposit), format_usd(BITCOIN_PRICE), format_usd(deposit)), None), // your deposit
                // Some(("50% of Today's Price Guaranteed", &format!("If Bitcoin is {}, I will withdraw {}\nIf Bitcoin is below {}, I will withdraw {}", format_usd(prediction), format_usd((multiple*0.7)*deposit), format_usd(BITCOIN_PRICE/2.0), format_usd(deposit/2.0)), None)),
                None
            ),
            false => ListItemSelector::new(ctx,
                ("15% Guaranteed Return", &format!("I will withdraw {}", format_usd(deposit*2.0)), None),
                ("Today's Price Guaranteed", &format!("If Bitcoin is {}, I will withdraw {}\nIf Bitcoin is below {}, I will withdraw {}", format_usd(prediction), format_usd((multiple*0.3)*deposit), format_usd(BITCOIN_PRICE), format_usd(deposit)), None), // your deposit
                Some(("50% of Today's Price Guaranteed", &format!("If Bitcoin is {}, I will withdraw {}\nIf Bitcoin is below {}, I will withdraw {}", format_usd(prediction), format_usd((multiple*0.7)*deposit), format_usd(BITCOIN_PRICE/2.0), format_usd(deposit/2.0)), None)),
                None
            ),
        };

        let button = Button::primary(ctx, "Continue", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let bumper = Bumper::single_button(ctx, button);
        let content = Content::new(Offset::Start, vec![Box::new(text), Box::new(selector)]);
        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Select Contract", None);
        SelectContract(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky)
    }
}

impl OnEvent for SelectContract {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let selector = self.1.content().find::<ListItemSelector>().unwrap();
            let contract = ctx.state().get_mut::<MintyContract>().expect("NO CONTRACT");
            match selector.index() {
                Some(0) if self.2 => contract.variant = ContractType::AdditonalReturn285,
                Some(1) if self.2 => contract.variant = ContractType::AdditonalReturn270,
                Some(2) if self.2 => contract.variant = ContractType::AdditonalReturn30,
                Some(0) => contract.variant = ContractType::GuaranteedReturn15,
                Some(1) => contract.variant = ContractType::TodaysPriceGuaranteed,
                Some(2) => contract.variant = ContractType::TodaysPriceGuaranteed50,
                _ => {}
            }
        }
        true
    }
}