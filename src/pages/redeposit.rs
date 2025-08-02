use pelican_ui::events::{OnEvent, Event, TickEvent};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{
    Page, Stack, Bumper,
    Header, AppPage,
    NavigateEvent, Button,
    IconButton, Offset, Content,
    TextInput, ButtonState,
    QuickActions, SetActiveInput,
};

use crate::{
    MintyContract, QRCodeDeposit, 
    ViewContract, SimilarContracts
};

use crate::components::TextInputMinty;

#[derive(Debug, Component)]
pub struct RedepositAddress(Stack, Page, 
    #[skip] ButtonState, 
    #[skip] bool, // is risky offer
    #[skip] bool, // is my own offer
    #[skip] bool, // was a matching offer
    #[skip] MintyContract,
);

impl AppPage for RedepositAddress {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 if !self.5 => Ok(Box::new(SimilarContracts::new(ctx, self.3, None))),
            // 0 => Ok(Box::new(ConfirmContract::new(ctx, self.3))),
            0 => Ok(Box::new(ViewContract::new(ctx, self.3, self.5, self.6))),
            // 1 => Ok(Box::new(ContractAccepted::new(ctx, self.4))),
            1 => Ok(Box::new(QRCodeDeposit::new(ctx, self.3, self.4, self.5, self.6))),
            _ => Err(self)
        }
    }
}

impl RedepositAddress {
    pub fn new(ctx: &mut Context, _address: Option<String>, is_risky: bool, is_mine: bool, was_match: bool, contract: MintyContract) -> Self {
        let button = Button::disabled(ctx, "Continue", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        let input = TextInputMinty::address(ctx);

        let paste = Button::secondary(ctx, Some("paste"), "Paste Clipboard", None, move |ctx: &mut Context| {
            let data = ctx.hardware.paste();
            ctx.trigger_event(SetActiveInput(data))
        }, Some("Pasted Clipboard".to_string()));

        let scan_qr = Button::secondary(ctx, Some("qr_code"), "Scan QR Code", None, |ctx: &mut Context| ctx.trigger_event(NavigateEvent(2)), None);
        // let contact = Button::secondary(ctx, Some("profile"), "Select Contact", None, |ctx: &mut Context| ctx.trigger_event(NavigateEvent(3)), None);

        let quick_actions = QuickActions::new(vec![paste, scan_qr]);
        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));

        let header = Header::stack(ctx, Some(back), "Send bitcoin", None);
        let bumper = Bumper::single_button(ctx, button);
        let content = Content::new(Offset::Start, vec![Box::new(input), Box::new(quick_actions)]);

        RedepositAddress(Stack::default(), Page::new(Some(header), content, Some(bumper)), ButtonState::Default, is_risky, is_mine, was_match, contract)
    }
}

impl OnEvent for RedepositAddress {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let input = &mut *self.1.content().find::<TextInput>().unwrap();
            let input_address = input.value().clone();

            if !input_address.is_empty() {
                // let (address, amount) = ("", None);// parse_btc_uri(input_address);
                // *input.value() = "address".to_string(); // PROBLEM
                // let address = SendAddress::new(address.to_string());
                // if let Some(b) = amount { ctx.state().set(&SendAmount::new(b)) }

                // match address.is_valid() {
                //     true => *input.error() = false,
                //     false => input.set_error(ctx, "Address is not valid.")
                // }

                // ctx.state().set(&address);
                ctx.state().get_mut::<MintyContract>().unwrap().address = input_address.clone();
            }

            let error = *input.error() || input_address.is_empty();
            let button = self.1.bumper().as_mut().unwrap().find::<Button>().unwrap();
            button.update_state(ctx, error, !error, &mut self.2);
        }
        true
    }
}
