use pelican_ui::{Theme, Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS, include_assets};
use pelican_ui::drawable::{Color, Drawable, Component};
use pelican_ui_std::{Stack, Interface};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::runtime::{Services, ServiceList};
use profiles::plugin::ProfilePlugin;
use profiles::service::{Name, ProfileService};
use profiles::components::AvatarContentProfiles;
use pelican_ui::ColorResources;
use pelican_ui::theme::{IllustrationColors, ButtonColorScheme, ButtonColors, BrandColor, TextColor, OutlineColor, BackgroundColor, ShadesColor, StatusColor};
use std::collections::HashMap;
use messages::service::{Rooms};

mod pages;
use pages::*;
mod components;
use components::*;
mod state;
use state::*;

// mod bdk;
// use bdk::BDKPlugin;
// mod msg;
// use msg::MSGPlugin;
// use ucp_rust::UCPPlugin;

pub struct MyApp;
impl Services for MyApp {
    fn services() -> ServiceList {
        let mut services = ServiceList::default();
        services.insert::<ProfileService>();
        services
    }
}

impl Plugins for MyApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![Box::new(ProfilePlugin::new(ctx))]
    }
}

impl Application for MyApp {
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> { App::new(ctx) }
}

start!(MyApp);

#[derive(Debug, Component)]
pub struct App(Stack, Interface);

impl App {
    pub fn new(ctx: &mut Context) -> Box<Self> {
        // let account_actions = Rc::new(RefCell::new(vec![IconButtonBitcoin::new(ctx), IconButtonMessages::new(ctx), IconButtonProfiles::block(ctx)]));
        // let messages_actions = account_actions.clone();
        // let navigation = vec![
        //     ("wallet", "Bitcoin".to_string(), None, Some(Box::new(|ctx: &mut Context| Box::new(BitcoinHome::new(ctx)) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>)),
        //     ("messages", "Messages".to_string(), None, Some(Box::new(move |ctx: &mut Context| Box::new(MessagesHome::new(ctx, messages_actions.clone())) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>)),
        //     ("door", "Rooms".to_string(), None, Some(Box::new(move |ctx: &mut Context| Box::new(RoomsHome::new(ctx, account_actions.clone())) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>)),
        // ];

        // let navigation_b = vec![
        //     ("profile", "My Account".to_string(), Some(AvatarContentProfiles::default()), Some(Box::new(|ctx: &mut Context| Box::new(Account::new(ctx)) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>))
        // ];

        ctx.state().set(MyContracts(Vec::new()));
        Self::theme(ctx);
        let home = MintyTerms::new(ctx);
        let interface = Interface::new(ctx, Box::new(home), None);
        Box::new(App(Stack::default(), interface))
    }

    pub fn theme(ctx: &mut Context) {
        ctx.assets.include_assets(include_assets!("./resources"));

        let mut theme = Theme::default(&mut ctx.assets);

        theme.brand.set_logomark(ctx, "brand/logo.svg");
        theme.brand.set_app_icon(ctx, "brand/app_icon.svg");
        theme.brand.set_wordmark(ctx, "brand/wordmark.svg");

        theme.icons.insert(ctx, "brand");

        theme.colors = ColorResources::new(
            BackgroundColor {
                primary: Color::from_hex("FFFFFF", 255),
                secondary: Color::from_hex("E6E6E6", 255),
            },
            OutlineColor {
                primary: Color::from_hex("78716C", 255),
                secondary: Color::from_hex("443F3F", 255),
            },
            StatusColor {
                success: Color::from_hex("3ccb5a", 255),
                warning: Color::from_hex("f5bd14", 255),
                danger: Color::from_hex("ff330a", 255),
            },
            TextColor {
                heading: Color::from_hex("000000", 255),
                primary: Color::from_hex("323232", 255),
                secondary: Color::from_hex("6A6969", 255),
            },
            BrandColor {
                primary: Color::from_hex("6DD495", 255),
                secondary: Color::from_hex("FFFFFF", 255),
            },
            ShadesColor::default(),
            ButtonColors {
                primary_default: ButtonColorScheme {
                    background: Color::from_hex("42DD94", 255),
                    label: Color::from_hex("FFFFFF", 255),
                    outline: Color::from_hex("FFFFFF", 0),
                },
                primary_disabled: ButtonColorScheme {
                    background: Color::from_hex("837A7A", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("FFFFFF", 0),
                },
                primary_hover: ButtonColorScheme {
                    background: Color::from_hex("33C781", 255),
                    label: Color::from_hex("FFFFFF", 255),
                    outline: Color::from_hex("FFFFFF", 0),
                },
                primary_selected: ButtonColorScheme {
                    background: Color::from_hex("33C781", 255),
                    label: Color::from_hex("FFFFFF", 255),
                    outline: Color::from_hex("FFFFFF", 0),
                },
                primary_pressed: ButtonColorScheme {
                    background: Color::from_hex("33C781", 255),
                    label: Color::from_hex("FFFFFF", 255),
                    outline: Color::from_hex("FFFFFF", 0),
                },

                secondary_default: ButtonColorScheme {
                    background: Color::from_hex("FFFFFF", 0),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("605E5E", 255),
                },
                secondary_disabled: ButtonColorScheme {
                    background: Color::from_hex("78716c", 255),
                    label: Color::from_hex("000000", 255),
                    outline:Color::from_hex("000000", 255),
                },
                secondary_hover: ButtonColorScheme {
                    background: Color::from_hex("E6E6E5", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("A49C9A", 255),
                },
                secondary_selected: ButtonColorScheme {
                    background: Color::from_hex("D6D6D5", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("A49C9A", 255),
                },
                secondary_pressed: ButtonColorScheme {
                    background: Color::from_hex("D6D6D5", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("A49C9A", 255),
                },

                ghost_default: ButtonColorScheme {
                    background: Color::from_hex("000000", 0),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("000000", 0),
                },
                ghost_disabled: ButtonColorScheme {
                    background: Color::from_hex("000000", 0),
                    label: Color::from_hex("78716c", 255),
                    outline: Color::from_hex("000000", 0),
                },
                ghost_hover: ButtonColorScheme {
                    background: Color::from_hex("262322", 255),
                    label: Color::from_hex("FFFFFF", 255),
                    outline: Color::from_hex("FFFFFF", 255),
                },
                ghost_selected: ButtonColorScheme {
                    background: Color::from_hex("D8D8D8", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("FFFFFF", 0),
                },
                ghost_pressed: ButtonColorScheme {
                    background: Color::from_hex("D8D8D8", 255),
                    label: Color::from_hex("000000", 255),
                    outline: Color::from_hex("FFFFFF", 0),
                },
            },
            IllustrationColors {
                colors: HashMap::new(),
            }
        );

        ctx.theme = theme;
    }
}

impl OnEvent for App {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let rooms = &ctx.state().get_or_default::<Rooms>().0;
            let any_unread = rooms.iter().any(|r| r.1.2.iter().any(|m| !m.is_read()));
            if let Some(mobile) = self.1.mobile() {
                if let Some(n) = mobile.navigator().as_mut() { n.inner().buttons()[1].show_flair(any_unread); }
            } else if let Some(desktop) = self.1.desktop() {
                if let Some(n) = desktop.navigator().as_mut() { n.buttons()[1].show_flair_left(any_unread); }
            }

            if ctx.state().get::<Name>().is_some() {
                self.1.desktop().as_mut().map(|d| d.navigator().as_mut().map(|nav| {
                    let me = ProfilePlugin::me(ctx).0;
                    nav.update_avatar(AvatarContentProfiles::from_orange_name(ctx, &me));

                    // let username = ProfilePlugin::username(ctx);
                    // let username = NameGenerator::display_name(username);
                    // nav.update_username(username)
                }));
            }
        }
        true
    }
}