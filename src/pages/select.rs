use pelican_ui::events::{OnEvent, Event, TickEvent};
use pelican_ui::drawable::{Align, Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Bumper,
    Header, AppPage,
    NavigateEvent, Button,
    IconButton, Offset, Content,
    ListItemSelector, TextStyle,
    ExpandableText, 
};

use bitcoin::format_usd;
use crate::{
    ContractType, MintyContract, 
    BITCOIN_PRICE, ConfirmContract,
    MintyHome, BitcoinDeposit
};

#[derive(Debug, Component)]
pub struct SelectContract(Stack, Page);

impl AppPage for SelectContract {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(BitcoinDeposit::new(ctx))),
            1 => Ok(Box::new(ConfirmContract::new(ctx))),
            _ => Err(self)
        }
    }
}

impl SelectContract {
    pub fn new(ctx: &mut Context) -> Self {
        let prediction = ctx.state().get_or_default::<MintyContract>().prediction;
        let deposit = ctx.state().get_or_default::<MintyContract>().deposited;
        let text_size = ctx.theme.fonts.size.h4;
        let multiple = prediction / BITCOIN_PRICE;
        let text = ExpandableText::new(ctx, &format!("I will deposit {}", format_usd(deposit)), TextStyle::Heading, text_size, Align::Center, None);
        let is_risky = ctx.state().get_mut_or_default::<MintyContract>().is_risky;
        
        let selector = match is_risky {
            true => ListItemSelector::new(ctx,
                ("100% Of Counterparty's Return", "You receive all of the price appreciation of your counterparty's Bitcoin after their guaranteed 15% annual return.", None),
                ("75% Of Counterparty's Return", "You receive 75% of the price appreciation of your counterparty's Bitcoin and protect them from a price drop.", None),
                Some(("35% Of Counterparty's Return", "You receive 35% of the price appreciation of your counterparty's Bitcoin and protect them from more than a 50% price drop.", None)),
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
        let content = Content::new(ctx, Offset::Start, vec![Box::new(text), Box::new(selector)]);
        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Select Contract", None);
        SelectContract(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}

impl OnEvent for SelectContract {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let selector = self.1.content().find::<ListItemSelector>().unwrap();
            let contract = ctx.state().get_mut::<MintyContract>().expect("NO CONTRACT");
            match selector.index() {
                Some(0) if contract.is_risky => contract.variant = ContractType::OfCounterpartyReturn100,
                Some(1) if contract.is_risky => contract.variant = ContractType::OfCounterpartyReturn75,
                Some(2) if contract.is_risky => contract.variant = ContractType::OfCounterpartyReturn35,
                Some(0) => contract.variant = ContractType::GuaranteedReturn15,
                Some(1) => contract.variant = ContractType::TodaysPriceGuaranteed,
                Some(2) => contract.variant = ContractType::TodaysPriceGuaranteed50,
                _ => {}
            }
        }
        true
    }
}