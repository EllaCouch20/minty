use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Header,
    NavigateEvent, IconButton,
    Offset, Content, Bumper,
    Button, AppPage,
};

use crate::{MintyContract, MintyHome};

use crate::components::DataItemMinty;

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
    pub fn new(ctx: &mut Context) -> Self {
        let contract = ctx.state().get_or_default::<MintyContract>().clone();
        let details = DataItemMinty::contract_details(ctx, &contract);

        let done = Button::close(ctx, "Done", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0))); 
        let bumper = Bumper::single_button(ctx, done);
        let content = Content::new(ctx, Offset::Start, vec![Box::new(details)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Contract details", None);
        ContractDetails(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}