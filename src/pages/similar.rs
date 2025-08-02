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

use crate::{MintyContract, RedepositAddress, ViewContract, ConfirmContract};

#[derive(Debug, Component)]
pub struct SimilarContracts(Stack, Page, #[skip] bool, #[skip] Option<MintyContract>);

impl OnEvent for SimilarContracts {}

impl AppPage for SimilarContracts {
    fn has_nav(&self) -> bool { false }
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> { 
        match index {
            0 => match self.3 {
                Some(contract) => Ok(Box::new(ViewContract::new(ctx, self.2, false, contract))),
                None => Ok(Box::new(ConfirmContract::new(ctx, self.2)))
            },
            1 => {
                let contract = ctx.state().get::<MintyContract>().unwrap().clone();
                Ok(Box::new(RedepositAddress::new(ctx, None, self.2, true, false, contract)))
            }
            // 2 => Ok(Box::new(ViewContract::new(ctx, self.2, false, ))),
            // 2 => Ok(Box::new(RedepositAddress::new(ctx, None, self.2, false, false))),
            _ => Err(self)
        }
    }
}

impl SimilarContracts {
    pub fn new(ctx: &mut Context, is_risky: bool, rejected: Option<MintyContract>) -> Self {
        let text_size = ctx.theme.fonts.size.sm;

        let text = "No existing contracts matched.\nSelect a different contract below or publish your offer.";
        let text = ExpandableText::new(ctx, text, TextStyle::Primary, text_size, Align::Center, None);

        // let list_items = ListItemGroup::new(vec![
        //     ListItemMinty::new(ctx, 560_000.0, 100_000.00, 2),
        //     ListItemMinty::new(ctx, 450_000.0, 100_000.00, 2),
        //     ListItemMinty::new(ctx, 740_000.0, 100_000.00, 2),
        //     ListItemMinty::new(ctx, 566_000.0, 100_000.00, 2),
        // ]);


        let button = Button::primary(ctx, "Publish My Offer", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(1)));
        
        let bumper = Bumper::single_button(ctx, button);
        let content = Content::new(Offset::Start, vec![Box::new(text)]);

        let back = IconButton::navigation(ctx, "left", |ctx: &mut Context| ctx.trigger_event(NavigateEvent(0)));
        let header = Header::stack(ctx, Some(back), "Similar contracts", None);
        SimilarContracts(Stack::default(), Page::new(Some(header), content, Some(bumper)), is_risky, rejected)
    }
}
