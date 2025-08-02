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

        // let my = ctx.state().get::<MintyContract>().unwrap().clone();

        let text = ExpandableText::new(ctx, "A matching contract has been found.\nAccept the offer, or reject to see similar offers.", TextStyle::Secondary, text_size, Align::Center, None);
        let prediction = DataItemMinty::confirm_prediction(ctx, is_risky, None);
        let deposit = DataItemMinty::contract_terms(ctx, is_risky, None);

        let accept = Button::primary(ctx, "Accept", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let reject = Button::secondary_expand(ctx, "Reject", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0))); 
        let bumper = Bumper::double_button(ctx, reject, accept);
        let content = Content::new(Offset::Start, vec![Box::new(text), Box::new(prediction), Box::new(deposit)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Matching contract", None);
        ViewContract(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky, was_match, contract)
    }
}
