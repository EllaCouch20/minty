use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Align, Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Bumper,
    Header, AppPage,
    NavigateEvent, Button,
    IconButton, Offset, Content,
    TextStyle, ExpandableText,
};

use crate::{
    MintyContract, RedepositAddress,
    SimilarContracts, ConfirmContract
};

use crate::components::DataItemMinty;

#[derive(Debug, Component)]
pub struct ViewMatchingContract(Stack, Page);
impl OnEvent for ViewMatchingContract {}

impl AppPage for ViewMatchingContract {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(ConfirmContract::new(ctx))),
            1 => Ok(Box::new(RedepositAddress::new(ctx))),
            2 => Ok(Box::new(SimilarContracts::new(ctx))),
            _ => Err(self)
        }
    }
}

impl ViewMatchingContract {
    pub fn new(ctx: &mut Context) -> Self {
        let text_size = ctx.theme.fonts.size.md;

        // let my = ctx.state().get::<MintyContract>().unwrap().clone();

        let contract = ctx.state().get_or_default::<MintyContract>().clone();
        let text = ExpandableText::new(ctx, "A matching contract has been found.\nAccept the offer, or reject to see similar offers.", TextStyle::Secondary, text_size, Align::Center, None);
        let prediction = DataItemMinty::confirm_prediction(ctx, contract.is_risky, None);
        let deposit = DataItemMinty::contract_terms(ctx, contract.is_risky, None);

        let accept = Button::primary(ctx, "Accept", move |ctx: &mut Context| {
            ctx.state().get_mut_or_default::<MintyContract>().accepted_contract = Some(Box::new(contract.clone()));
            ctx.trigger_event(NavigateEvent(1));
        });

        let reject = Button::secondary_expand(ctx, "Reject", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(2))); 
        let bumper = Bumper::double_button(ctx, reject, accept);
        let content = Content::new(Offset::Start, vec![Box::new(text), Box::new(prediction), Box::new(deposit)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Matching contract", None);
        ViewMatchingContract(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}


#[derive(Debug, Component)]
pub struct ViewSimilarContract(Stack, Page);
impl OnEvent for ViewSimilarContract {}

impl AppPage for ViewSimilarContract {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(ConfirmContract::new(ctx))),
            1 => Ok(Box::new(RedepositAddress::new(ctx))),
            _ => Err(self)
        }
    }
}

impl ViewSimilarContract {
    pub fn new(ctx: &mut Context) -> Self {
        let text_size = ctx.theme.fonts.size.md;
        let contract = ctx.state().get_or_default::<MintyContract>().clone();

        let prediction = DataItemMinty::confirm_prediction(ctx, contract.is_risky, None);
        let deposit = DataItemMinty::contract_terms(ctx, contract.is_risky, None);

        let accept = Button::primary(ctx, "Accept Offer", move |ctx: &mut Context| {
            ctx.state().get_mut_or_default::<MintyContract>().accepted_contract = Some(Box::new(contract.clone()));
            ctx.trigger_event(NavigateEvent(1))
        });

        let bumper = Bumper::single_button(ctx, accept);
        let content = Content::new(Offset::Start, vec![Box::new(prediction), Box::new(deposit)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Matching contract", None);
        ViewSimilarContract(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}
