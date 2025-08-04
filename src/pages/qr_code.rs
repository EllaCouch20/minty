use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Align, Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Bumper,
    Header, AppPage,
    NavigateEvent, Button,
    IconButton, Offset, Content,
    TextStyle, ExpandableText, QRCode
};

use crate::{
    MyContracts, MintyContract, 
    MintyPlugin, Success,
    RedepositAddress,
};

use crate::service::MintyRequest;

#[derive(Debug, Component)]
pub struct QRCodeDeposit(Stack, Page);
impl OnEvent for QRCodeDeposit {}

impl AppPage for QRCodeDeposit {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(RedepositAddress::new(ctx))),
            1 => Ok(Box::new(Success::new(ctx))),
            _ => Err(self)
        }
    }
}

impl QRCodeDeposit {
    pub fn new(ctx: &mut Context) -> Self {
        let text_size = ctx.theme.fonts.size.md;
        let address = "Why are you even trying to deposit Bitcoin here???";

        let qr_code = QRCode::new(ctx, address);
        let text = ExpandableText::new(ctx, "Scan to deposit bitcoin and accept this contract.", TextStyle::Secondary, text_size, Align::Center, None);
        let content = Content::new(Offset::Center, vec![Box::new(qr_code), Box::new(text)]);

        let close = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(close), "Deposit bitcoin", None);

        let contract = ctx.state().get_or_default::<MintyContract>().clone();

        let contract_c = contract.clone();
        let button = Button::primary(ctx, "Skip", move |ctx: &mut Context| {
            if contract.matched_with.is_none() && contract.accepted_contract.is_none() {
                let mut guard = ctx.get::<MintyPlugin>();
                let (plugin, ctx) = guard.get();
                let my_contract = ctx.state().get::<MintyContract>().unwrap();
                plugin.request(MintyRequest::CreateContract(my_contract.clone()));
                ctx.trigger_event(NavigateEvent(1))
            } else {
                let contracts = ctx.state().get_mut::<MyContracts>().unwrap();
                contracts.0.iter_mut().for_each(|c| {
                    if *c == contract_c.clone() {
                        c.accepted = true;
                    }
                });
                ctx.trigger_event(NavigateEvent(1))
            }
        });
        let bumper = Bumper::single_button(ctx, button);
        QRCodeDeposit(Stack::default(), Page::new(Some(header), content, Some(bumper)))
    }
}
