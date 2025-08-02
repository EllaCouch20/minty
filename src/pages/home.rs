use pelican_ui::events::{OnEvent, Event};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Header,
    Content, Bumper,
    NavigateEvent, Button,
    Offset, ListItemGroup,
    AppPage,
};

use crate::{
    MintyContract, SetContractEvent, 
    MyContracts, BitcoinDeposit, 
    ContractDetails,
};

use crate::components::ListItemMinty;
use bitcoin::components::AmountDisplay;
use bitcoin::format_usd;

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

        let amount = ctx.state().get::<MyContracts>().map(|con| con.0.iter().map(|c| c.expected_amt).sum::<f64>().max(0.0)).unwrap_or(0.0);
        let amount = if amount == 0.0 {"$0.00".to_string()} else {format_usd(amount)};
        let display = AmountDisplay::new(ctx, &amount, "Expected contracts value");
        let mut content: Vec<Box<dyn Drawable>> = vec![Box::new(display)];
        let mut offset = Offset::Center;

        if let Some(contracts) = ctx.state().get::<MyContracts>() {
            if !contracts.0.is_empty() {
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
