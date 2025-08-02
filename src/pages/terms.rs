use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Align, Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Bumper, 
    AppPage, NavigateEvent, 
    Button, Offset, Content,
    TextStyle, ExpandableText,
    Brand
};

use crate::MintyHome;

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