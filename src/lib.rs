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
use pelican_ui::{IllustrationColors, ButtonColorScheme, ButtonColors, BrandColor, TextColor, OutlineColor, BackgroundColor, StatusColor};
use std::collections::HashMap;
use pelican_ui::hardware::ApplicationSupport;
// use messages::service::{Rooms};

// use serde::{Deserialize, Serialize};
use std::{
    fs,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use tempfile::NamedTempFile;

pub mod pages;
pub use pages::*;
mod components;
mod state;
use state::*;

mod plugin;
use plugin::MintyPlugin;
mod service;
use service::MintyService;

use pelican_ui_std::AppPage;
use profiles::pages::Account;

pub const BITCOIN_PRICE: f64 = 118_000.00;

// use bitcoin::components::IconButtonBitcoin;
// use messages::components::IconButtonMessages;
// use profiles::components::IconButtonProfiles;

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
        services.insert::<MintyService>();
        services
    }
}

impl Plugins for MyApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![Box::new(ProfilePlugin::new(ctx)), Box::new(MintyPlugin::new(ctx))]
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
        let navigation = vec![
            ("home", "Home".to_string(), None, Some(Box::new(|ctx: &mut Context| Box::new(MintyTerms::new(ctx)) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>)),
        ];

        let navigation_b = vec![
            ("profile", "My Account".to_string(), Some(AvatarContentProfiles::default()), Some(Box::new(|ctx: &mut Context| Box::new(Account::new(ctx)) as Box<dyn AppPage>) as Box<dyn FnMut(&mut Context) -> Box<dyn AppPage>>))
        ];

        let storage_path = ApplicationSupport::get().unwrap();
        fs::create_dir_all(&storage_path).unwrap();
        let path = storage_path.join("contracts.json");
        let _ = fs::remove_file(&path);
        File::create(&path).unwrap();
        // let mut contracts = Self::load_contracts(&path);
 
        ctx.state().set(MyContracts(Vec::new()));
        Self::theme(ctx);
        let home = MintyTerms::new(ctx);
        let interface = Interface::new(ctx, Box::new(home), Some((0_usize, navigation, navigation_b)), None);
        Box::new(App(Stack::default(), interface))
    }

    pub fn save_contracts<P: AsRef<Path>>(path: P, contracts: &Vec<MintyContract>) {
        let path = path.as_ref();
        let bytes = serde_json::to_vec_pretty(contracts).expect("Could not vec to pretty");

        let mut tmp = NamedTempFile::new_in(path.parent().unwrap_or_else(|| Path::new("."))).expect("Could not write temp");
        tmp.write_all(&bytes).expect("Could not write all");
        tmp.flush().expect("Could not flush");
        tmp.persist(path).expect("Colud not persist");
    }

       // contracts.push(Contract { id: 1, name: "Foo".into(), terms: "Bar".into() });
        // save_contracts(&path, &contracts).unwrap();

    pub fn load_contracts<P: AsRef<Path>>(path: P) -> Vec<MintyContract> {
        let path = path.as_ref();
        if !path.exists() {
            return Vec::new();
        }
        let file = File::open(path).expect("Could not open path");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Could not read from reader")
    }

    pub fn theme(ctx: &mut Context) {
        ctx.assets.include_assets(include_assets!("./resources"));

        let mut theme = Theme::default(&mut ctx.assets);

        theme.brand.set_logomark(ctx, "brand/logo.svg");
        theme.brand.set_app_icon(ctx, "brand/app_icon.svg");
        theme.brand.set_wordmark(ctx, "brand/wordmark.svg");
        theme.brand.set_error(ctx, "brand/wordmark.svg");

        theme.icons.insert(ctx, "brand");

        theme.colors = ColorResources::new(
            BackgroundColor {
                primary: Color::from_hex("FFFFFF", 255),
                secondary: Color::from_hex("B5B5B5", 255),
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
                secondary: Color::from_hex("2d2d2d", 255),
            },
            BrandColor {
                primary: Color::from_hex("6DD495", 255),
                secondary: Color::from_hex("FFFFFF", 255),
            },
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
                    background: Color::from_hex("262322", 255),
                    label: Color::from_hex("FFFFFF", 255),
                    outline: Color::from_hex("FFFFFF", 255),
                },
                ghost_pressed: ButtonColorScheme {
                    background: Color::from_hex("262322", 255),
                    label: Color::from_hex("FFFFFF", 255),
                    outline: Color::from_hex("FFFFFF", 255),
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
            // let contracts = ctx.state().get::<MyContracts>().unwrap().0.clone();
            // let storage_path = ApplicationSupport::get().unwrap();
            // std::fs::create_dir_all(&storage_path).unwrap();
            // let path = storage_path.join("contracts.json");
            // Self::save_contracts(&path, &contracts);

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