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
    MintyHome,
};

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
    pub fn new(ctx: &mut Context, is_risky: bool) -> Self {
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