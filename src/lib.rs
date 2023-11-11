pub mod client;
pub mod network_object;
pub mod server;
pub mod utils;

pub use bevy::prelude::*;
use bevy::utils::HashMap;
pub use bevy_rapier2d::prelude::*;
use network_object::NetworkObjectId;
use utils::{default_entity::DefaultEntity, newtype::network_sprite::NetworkSprite};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ClientMsg {
    Hop(Vec2),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServerMsg {
    SyncNetworkObject(NetworkObjectId, Transform, NetworkSprite),
    SpawnNetworkObject(NetworkObjectId, Transform, NetworkSprite),
    DespawnNetworkObject(NetworkObjectId),
}

pub const PROTOCOL_ID: u64 = 0;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Component, serde::Serialize, serde::Deserialize,
)]
pub struct PlayerId(pub u64);

#[derive(Default, Clone, Resource, Debug)]
pub struct PlayerEntities {
    pub player_id_to_entity: HashMap<PlayerId, DefaultEntity>,
}
#[derive(Default, Component, Clone)]
pub struct NetworkObject;
