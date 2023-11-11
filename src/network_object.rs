use bevy::{prelude::*, utils::HashMap};
use bevy_renet::{
    renet::{DefaultChannel, RenetServer},
    transport::NetcodeServerPlugin,
};

use crate::{utils::newtype::network_sprite::NetworkSprite, ServerMsg};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Component, serde::Serialize, serde::Deserialize,
)]
pub struct NetworkObjectId(pub u32);

#[derive(Debug, Clone, Default, Resource)]
pub struct NetworkObjectMappingServer {
    pub obj_entity: HashMap<NetworkObjectId, Entity>,
    pub entity_obj: HashMap<Entity, NetworkObjectId>,
    pub next: u32,
}
impl NetworkObjectMappingServer {
    // INFO: return None if no more network id can be allocated
    pub fn add_obj(&mut self, entity: Entity) -> Option<NetworkObjectId> {
        if self.next + 1 == u32::MAX {
            return None;
        }
        let id = self.next;

        self.obj_entity.insert(NetworkObjectId(self.next), entity);
        self.entity_obj.insert(entity, NetworkObjectId(self.next));

        self.next += 1;
        return Some(NetworkObjectId(id));
    }
    pub fn remove_obj(&mut self, val: RemoveNetworkObjectEnum) -> Option<()> {
        match val {
            RemoveNetworkObjectEnum::Entity(x) => {
                let obj = self.entity_obj.get(&x)?.clone();

                self.obj_entity.remove(&obj);
                self.entity_obj.remove(&x);
            }
            RemoveNetworkObjectEnum::NetworkObjectId(x) => {
                let entity = self.obj_entity.get(&x)?.clone();
                self.obj_entity.remove(&x);
                self.entity_obj.remove(&entity);
            }
        };
        Some(())
    }
    // INFO: same as add_obj but it also notifies the players
    pub fn add_obj_and_notify(
        &mut self,
        entity: Entity,
        server: &mut ResMut<RenetServer>,
        q_transform: &Query<&Transform>,
        q_sprite: &Query<&Sprite>,
    ) -> Option<NetworkObjectId> {
        let Some(net_id) = self.add_obj(entity) else {
            panic!("not enough network id")
        };
        //let message = ServerMsg::SpawnNetworkObject(net_id);
        let message = serde_cbor::to_vec(&ServerMsg::SpawnNetworkObject(
            net_id,
            q_transform.get(entity).unwrap().clone(),
            q_sprite.get(entity).unwrap().clone().into(),
        ))
        .unwrap();

        server.broadcast_message(DefaultChannel::ReliableUnordered, message);

        Some(net_id)
    }
    pub fn remove_obj_and_notify(
        &mut self,
        val: RemoveNetworkObjectEnum,

        server: &mut ResMut<RenetServer>,
    ) -> Option<()> {
        match val {
            RemoveNetworkObjectEnum::Entity(x) => {
                let obj = self.entity_obj.get(&x)?.clone();

                server.broadcast_message(
                    DefaultChannel::ReliableUnordered,
                    serde_cbor::to_vec(&ServerMsg::DespawnNetworkObject(obj)).unwrap(),
                );
            }
            RemoveNetworkObjectEnum::NetworkObjectId(x) => {
                server.broadcast_message(
                    DefaultChannel::ReliableUnordered,
                    bincode::serialize(&ServerMsg::DespawnNetworkObject(x)).unwrap(),
                );
            }
        };
        self.remove_obj(val.clone());
        Some(())
    }
}
#[derive(Debug, Clone)]
pub enum RemoveNetworkObjectEnum {
    Entity(Entity),
    NetworkObjectId(NetworkObjectId),
}

#[derive(Debug, Clone, Default, Resource)]
pub struct NetworkObjectMappingClient {
    pub obj_entity: HashMap<NetworkObjectId, Entity>,
    pub entity_obj: HashMap<Entity, NetworkObjectId>,
}

impl NetworkObjectMappingClient {
    // INFO: return None if no more network id can be allocated
    pub fn add_obj(&mut self, entity: Entity, net_obj: NetworkObjectId) {
        self.obj_entity.insert(net_obj, entity);
        self.entity_obj.insert(entity, net_obj);
    }
    // INFO: return None if the requested identifier doesnt exist
    pub fn remove_obj(&mut self, val: RemoveNetworkObjectEnum) -> Option<()> {
        match val {
            RemoveNetworkObjectEnum::Entity(x) => {
                let obj = self.entity_obj.get(&x)?.clone();

                self.obj_entity.remove(&obj);
                self.entity_obj.remove(&x);
            }
            RemoveNetworkObjectEnum::NetworkObjectId(x) => {
                let entity = self.obj_entity.get(&x)?.clone();
                self.obj_entity.remove(&x);
                self.entity_obj.remove(&entity);
            }
        };
        Some(())
    }
}
