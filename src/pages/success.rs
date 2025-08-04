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
    AvatarIconStyle, AvatarContent,
    Avatar, 
};

use bitcoin::format_usd;
use crate::{ MintyContract, MintyHome };

#[derive(Debug, Component)]
pub struct Success(Stack, Page);

impl OnEvent for Success {}

impl AppPage for Success {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => Ok(Box::new(MintyHome::new(ctx))),
            _ => Err(self)
        }
    }
}

impl Success {
    pub fn new(ctx: &mut Context) -> Self {
        let contract = ctx.state().get_or_default::<MintyContract>().clone();
        let text_size = ctx.theme.fonts.size.h4;
        let icon = Avatar::new(ctx, AvatarContent::Icon("brand", AvatarIconStyle::Brand), None, false, 72.0, None);
        
        let accepted_contract = contract.accepted_contract.is_some();

        let (subtext, title) = match accepted_contract {
            true => ("Your contract has been published".to_string(), "Contract published"),
            false => (format!("You accepted a contract for {}", &format_usd(contract.deposited)), "Contract accepted"),
        };

        let text = ExpandableText::new(ctx, &subtext, TextStyle::Heading, text_size, Align::Center, None);
        let done = Button::secondary_expand(ctx, "Done", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let bumper = Bumper::single_button(ctx, done);
        let content = Content::new(Offset::Center, vec![Box::new(icon), Box::new(text)]);
        let close = IconButton::close(ctx, |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(close), title, None);
        Success(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}
