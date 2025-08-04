use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Header,
    NavigateEvent, IconButton,
    Content, Offset, Bumper,
    Button, AppPage
};

use crate::{
    MyContracts, MintyContract,
    BITCOIN_PRICE, SimilarContracts,
    ViewMatchingContract, BitcoinDeposit,
    BitcoinPrediction, SelectContract,
};

use crate::components::DataItemMinty;

#[derive(Debug, Component)]
pub struct ConfirmContract(Stack, Page, #[skip] Option<MintyContract>);
impl OnEvent for ConfirmContract {}

impl AppPage for ConfirmContract {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(SelectContract::new(ctx))),
            1 => Ok(Box::new(BitcoinPrediction::new(ctx))),
            2 => Ok(Box::new(BitcoinDeposit::new(ctx))),
            3 => match self.2 {
                Some(contract) => Ok(Box::new(ViewMatchingContract::new(ctx))),
                None => Ok(Box::new(SimilarContracts::new(ctx))),
            }
            _ => Err(self)
        }
    }
}

impl ConfirmContract {
    pub fn new(ctx: &mut Context) -> Self {


        // for risky, the less risky side's deposit must be higher than the risky side's (expected withdrawal amount - deposit amount)
        // for risky, the less risky side's 'expected upside' must be lower than 2 - (risky's expected withdraw - risky's price prediction)
        // expected upside = your/their withraw \over\ your/their price prediction

        // for less, the minimum deposit is met. minimum deposit = (current price / insured amount) - my deposit
        // for less, their maximum upside is lower than 2 - (risky's expected withdraw - risky's price prediction)

        let mine = ctx.state().get::<MintyContract>().unwrap().clone();
        let confirm_prediction = DataItemMinty::confirm_prediction(ctx, mine.is_risky, Some(1));
        let contract_terms = DataItemMinty::contract_terms(ctx, mine.is_risky, Some((0, 2)));

        let mut match_found = None;
        ctx.state().get::<MyContracts>().unwrap().0.iter().filter(|c| !c.accepted).for_each(|theirs| {
            if mine.is_risky {
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
        ConfirmContract(Stack::default(), Page::new(Some(header), content, Some(bumper)), match_found)
    }
}
