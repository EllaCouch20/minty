// use pelican_ui::air::Id;
use pelican_ui::runtime;
use pelican_ui::{Context, Plugin};
// use serde_json::{Value, json};
// use std::hash::{DefaultHasher, Hasher, Hash};

use crate::service::{MintyRequest, MintyService};

pub struct MintyPlugin(runtime::Context);
impl Plugin for MintyPlugin {
    fn new(ctx: &mut Context) -> Self {
        MintyPlugin(ctx.runtime.clone())
    }
}
impl MintyPlugin {
    pub fn request(&mut self, request: MintyRequest) {
        self.0.send::<MintyService>(&request)
    }

    // pub fn create_message(ctx: &mut Context, id: Id, message: Message) {
    //     let mut guard = ctx.get::<MintyPlugin>();
    //     let plugin = guard.get().0;
    //     plugin.request(RoomsRequest::CreateMessage(id, message));
    // }
}
