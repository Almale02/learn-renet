use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};

use crate::{
    network_object::{NetworkObjectMappingClient, RemoveNetworkObjectEnum},
    utils::newtype::network_sprite::NetworkSprite,
    ServerMsg,
};

pub fn handle_request_client_ordered(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut net_obj_mapping: ResMut<NetworkObjectMappingClient>,
) {
    while let Some(req) = client.receive_message(DefaultChannel::ReliableUnordered) {
        let request = serde_cbor::from_slice::<ServerMsg>(&req).unwrap();

        match request {
            ServerMsg::SpawnNetworkObject(net_obj, transfrom, sprite) => {
                let id = commands
                    .spawn(
                        (SpriteBundle {
                            sprite: sprite.into(),
                            transform: transfrom,
                            ..default()
                        }),
                    )
                    .id();
                net_obj_mapping.add_obj(id, net_obj).clone()
            }
            ServerMsg::DespawnNetworkObject(net_obj) => {
                let id = net_obj_mapping.obj_entity.get(&net_obj).unwrap();

                commands.entity(*id).despawn_recursive();
                net_obj_mapping.remove_obj(RemoveNetworkObjectEnum::NetworkObjectId(net_obj));
            }
            _ => (),
        }
    }
}
pub fn handle_request_client_unreliable(
    mut q_transform: Query<&mut Transform>,
    mut q_sprite: Query<&mut Sprite>,
    net_obj_mapping: Res<NetworkObjectMappingClient>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(req) = client.receive_message(DefaultChannel::Unreliable) {
        let request = serde_cbor::from_slice::<ServerMsg>(&req).unwrap();

        match request {
            ServerMsg::SyncNetworkObject(obj_id, transform, sprite) => {
                if net_obj_mapping.obj_entity.get(&obj_id).is_none() {
                    continue;
                }
                let id = net_obj_mapping.obj_entity.get(&obj_id).unwrap();

                if q_transform.get(*id).is_err() {
                    continue;
                }
                *q_transform.get_mut(*id).unwrap() = transform;
                *q_sprite.get_mut(*id).unwrap() = sprite.into();
            }
            _ => (),
        }
    }
}
