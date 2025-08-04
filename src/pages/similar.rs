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
    ViewMatchingContract, ConfirmContract,
};

#[derive(Debug, Component)]
pub struct SimilarContracts(Stack, Page);
impl OnEvent for SimilarContracts {}

impl AppPage for SimilarContracts {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(ViewMatchingContract::new(ctx))),
            2 => Ok(Box::new(RedepositAddress::new(ctx))),
            _ => Err(self)
        }
    }
}

impl SimilarContracts {
    pub fn new(ctx: &mut Context) -> Self {

        let text_size = ctx.theme.fonts.size.sm;

        let text = "No existing contracts matched.\nSelect a different contract below or publish your offer.";
        let text = ExpandableText::new(ctx, text, TextStyle::Primary, text_size, Align::Center, None);

        let button = Button::primary(ctx, "Publish My Offer", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        
        let bumper = Bumper::single_button(ctx, button);
        let content = Content::new(Offset::Start, vec![Box::new(text)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Similar contracts", None);
        SimilarContracts(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}
