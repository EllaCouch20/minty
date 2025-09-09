// use std::collections::BTreeMap;
// use std::sync::LazyLock;
use std::time::Duration;
use pelican_ui::hardware::Cache;
use pelican_ui::runtime::{Services, Service, ServiceList, ThreadContext, async_trait, self};
use pelican_ui::{hardware};
use pelican_ui::State;
// use pelican_ui::air::{OrangeName, Id, Service as AirService, Protocol, Validation, ChildrenValidation, HeaderInfo, RecordPath, Permissions};
// use pelican_ui_std::AvatarContent;
// use messages::components::AvatarContentMessages;

// use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
// use uuid::Uuid;

use crate::{MintyContract, MyContracts};

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct Message(String, DateTime<Utc>, OrangeName, bool);
// impl Message {
//     pub fn from(message: String, author: OrangeName) -> Self {
//         Message(message, Utc::now(), author, false)
//     }

//     pub fn invisible(author: OrangeName) -> Self {
//         Message("__system__joined".to_string(), Utc::now(), author, true)
//     }

//     pub fn author(&self) -> &OrangeName {&self.2}
//     pub fn timestamp(&self) -> &DateTime<Utc> {&self.1}
//     pub fn message(&self) -> &String {&self.0}
//     pub fn is_read(&self) -> &bool {&self.3}
//     pub fn read(&mut self, status: bool) {self.3 = status}
// }

// #[derive(Serialize, Deserialize, Default, Clone, Debug)]
// pub struct Rooms(pub Vec<(Uuid, Room)>);

// impl Rooms {
//     pub fn rooms(&self) -> Vec<Room> {
//         self.0.clone().into_iter().map(|(_, r)| r).collect()
//     }
//     pub fn get(&mut self, id: Id) -> Option<&mut Room> {
//         self.0.iter_mut().find(|(_, i)| *i.0 == *id).map(|(_, r)| r)
//     }
// }

// pub type Room = (Id, Vec<OrangeName>, Vec<Message>);

// static ROOMS: LazyLock<Id> = LazyLock::new(|| Id::hash(&"RoomsV1".to_string()));
// static MESSAGES: LazyLock<Id> = LazyLock::new(|| Id::hash(&"MessagesV1".to_string()));
// static CONTRACTS: LazyLock<Id> = LazyLock::new(|| Id::hash(&"MintyContractsV1".to_string()));

// const ROOMS_PERMISSIONS: Permissions = Permissions::new(Some((true, true)), None, BTreeMap::new());
// const MESSAGES_PERMISSIONS: Permissions = Permissions::new(None, None, BTreeMap::new());

// static ROOMS_PROTOCOL: LazyLock<Protocol> = LazyLock::new(|| {
//     let cv = ChildrenValidation::new(vec![*MESSAGES], true, true, false);
//     let validation = Validation::new(Some(cv), None, BTreeMap::new(), false);
//     let header = HeaderInfo::new(None, BTreeMap::new(), Vec::new());
//     Protocol::new(validation, header, *ROOMS)
// });

// static MESSAGES_PROTOCOL: LazyLock<Protocol> = LazyLock::new(|| {
//     let validation = Validation::new(None, None, BTreeMap::new(), false);
//     let header = HeaderInfo::new(None, BTreeMap::new(), Vec::new());
//     Protocol::new(validation, header, *MESSAGES)
// });

#[derive(Serialize, Deserialize, Debug)]
pub enum MintyRequest {
    // CreateRoom(Uuid),
    CreateContract(MintyContract),
    // Share(Id, OrangeName),
}

#[derive(Debug)]
pub struct MintyService{
}

impl Services for MintyService {
    fn services() -> ServiceList {
        let mut services = ServiceList::default();
        services.insert::<MintySync>();
        services
    }
}

#[async_trait]
impl Service for MintyService {
    type Send = MintyContract;
    type Receive = MintyRequest;

    async fn new(_hardware: &mut hardware::Context) -> Self {
        MintyService{
        }
    }

    async fn run(&mut self, ctx: &mut ThreadContext<Self::Send, Self::Receive>) -> Result<Option<Duration>, runtime::Error> {
        // let cache = &mut MintyCache::from_cache(&mut ctx.hardware.cache).await;
        while let Some((_, request)) = ctx.get_request() {
            match request {
            //     MintyRequest::CreateRoom(uuid) => {
            //         while let (_, Some(_)) = AirService::create_private(ctx, RecordPath::root(), ROOMS_PROTOCOL.clone(), cache.contracts_idx, ROOMS_PERMISSIONS, serde_json::to_vec(&uuid)?).await? {
            //             cache.contracts_idx += 1;
            //         }
            //     },
                MintyRequest::CreateContract(contract) => {
                    println!("CREATING CONTRACT... {contract:?}");
                    // (contract);
                    ctx.callback(contract);
                    // while let (_, Some(_)) = AirService::create_private(ctx, RecordPath::root().join(*CONTRACTS), ROOMS_PROTOCOL.clone(), cache.contracts_idx, ROOMS_PERMISSIONS, serde_json::to_vec(&contract)?).await? {
                    //    cache.contracts_idx += 1;
                    // }
                },
            //     MintyRequest::Share(room, name) => {
            //         let message = Message::invisible(name.clone());
            //         let path = RecordPath::root().join(room);
            //         AirService::share(ctx, name, ROOMS_PERMISSIONS, path).await?;
            //         let mut x = cache.rooms.get(&RecordPath::root().join(room)).unwrap().2;
            //         while let (_, Some(_)) = AirService::create_private(ctx, RecordPath::root().join(room), MESSAGES_PROTOCOL.clone(), x, MESSAGES_PERMISSIONS, serde_json::to_vec(&message)?).await? {
            //             x += 1;
            //         }
            //     },
            }
        }

        Ok(Some(Duration::from_millis(16)))
    }

    fn callback(state: &mut State, response: Self::Send) {
        let mut contracts = state.get::<MyContracts>().unwrap().0.clone();
        contracts.push(response);
        state.set(MyContracts(contracts));
        // let mut rooms = state.get::<Rooms>().0;
        // // if response.2 {state.set(&Name(Some(response.0.clone())));}
        // rooms.insert(response.0, response.1);
        // state.set(&Rooms(rooms));
        // let mut contracts = state.get::<MyContracts>().0;
        // contracts.insert(contract);
        // state.set(MyContracts(contract));
    }
}

#[derive(Debug)]
pub struct MintySync{
    cache: MintyCache,
    _init: bool 
}

impl Services for MintySync {}

#[async_trait]
impl Service for MintySync {
    type Send = Vec<MintyContract>;
    type Receive = ();

    async fn new(hardware: &mut hardware::Context) -> Self {
        MintySync{
            cache: MintyCache::from_cache(&mut hardware.cache).await,
            _init: false
        }
    }

    async fn run(&mut self, ctx: &mut ThreadContext<Self::Send, Self::Receive>) -> Result<Option<Duration>, runtime::Error> {
        // let mut mutated = false;

        // for (_, path) in AirService::receive(ctx, self.cache.datetime).await?.into_iter() {
        //     // let uuid: Uuid = serde_json::from_slice(&AirService::read_private(ctx, path.clone()).await?.unwrap().0.payload).unwrap();
        //     // self.cache.rooms.entry(path).or_insert((uuid, vec![], 0));
        //     // mutated = true;

        //     println!("Creating pointer.");
        //     let mut x = self.cache.contracts_idx;
        //     while let (_, Some(_)) = AirService::create_pointer(ctx, RecordPath::root(), path.clone(), x).await? {
        //         x += 1;
        //     }
        //     println!("Done creating pointers.");
        //     mutated = true;
        // }

        // println!("Done receiving.");

        // self.cache.datetime = chrono::Utc::now();

        // while let (path, Some(_)) = AirService::discover(ctx, RecordPath::root().join(*CONTRACTS), self.cache.contracts_idx, vec![ROOMS_PROTOCOL.clone()]).await? {
        //     println!("Discovering...");
        //     if let Some(path) = path {
        //         if let Ok(contract) = serde_json::from_slice::<MintyContract>(&AirService::read_private(ctx, path.clone()).await?.unwrap().0.payload) {
        //             println!("Contract: {:?}...", contract);
        //             self.cache.contracts.entry(path).or_insert(MintyContract::empty());
        //             mutated = true;
        //         } else {println!("_--- PATH HAD NO CONTRACT ---_");}
        //     }
        //     self.cache.contracts_idx += 1;
        // }
        // println!("Done discovering.");

        // for (room, (_, messages, index)) in &mut self.cache.rooms {
        //     while let (path, Some(_)) = AirService::discover(ctx, room.clone(), *index, vec![MESSAGES_PROTOCOL.clone()]).await? {
        //         if let Some(path) = path {
        //             if let Ok(message) = serde_json::from_slice(&AirService::read_private(ctx, path).await?.unwrap().0.payload) {
        //                 messages.insert(*index as usize, message);
        //                 mutated = true;
        //             }
        //         }
        //         *index += 1;
        //     }
        // }

        // println!("Done messages.");
        
        // if mutated || !self.init {
        //     self.init = true;
        // ctx.callback(self.cache.contracts.clone());
        //     println!("Callback done.");
        // }

        // println!("Done updating.");
        self.cache.cache(&mut ctx.hardware.cache).await;
        // println!("DONE");
        Ok(Some(Duration::from_secs(1)))
    }

    fn callback(_state: &mut State, _response: Self::Send) {
        // println!("Callback... {:?}", response);
        // let mut contracts = state.get::<MyContracts>().0;
        // contracts.insert(response);
        // state.set(MyContracts(contract));
        // state.set(MyContracts(response))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MintyCache {
    pub contracts_idx: u32,
    pub contracts: Vec<MintyContract>,
    pub datetime: DateTime<Utc>,
}

impl MintyCache {
    pub async fn cache(&self, cache: &mut Cache) {
        // let other = cache.get::<Cache>("MintyCache").await;
        cache.set("MintyCache", self).await;
    }

    pub async fn from_cache(cache: &mut Cache) -> Self {
        cache.get("MintyCache").await
    }

    // pub fn merge(self, other: Self) -> Self {
    //     Cache {
    //         contracts_idx: self.contracts_idx.max(other.contracts_idx),
    //         rooms: self.rooms.extend(other.)
    //     }
    // }
}

impl Default for MintyCache {
    fn default() -> Self {
        // println!("MintyCache as default");
        MintyCache {
            contracts_idx: 0,
            contracts: Vec::new(),
            datetime: DateTime::UNIX_EPOCH,
        }
    }
}