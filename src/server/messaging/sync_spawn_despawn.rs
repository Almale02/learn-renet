use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::{
    network_object::{NetworkObjectMappingServer, RemoveNetworkObjectEnum},
    NetworkObject,
};

pub fn sync_spawn_net_obj(
    mut commands: Commands,
    q_transform: Query<&Transform>,
    q_sprite: Query<&Sprite>,
    q_added: Query<Entity, Added<NetworkObject>>,
    mut mapping: ResMut<NetworkObjectMappingServer>,
    mut server: ResMut<RenetServer>,
) {
    for id in &q_added {
        let net_id = mapping
            .add_obj_and_notify(id, &mut server, &q_transform, &q_sprite)
            .unwrap();
        commands.entity(id).insert(net_id);
    }
}
pub fn sync_despawn_net_obj(
    mut commands: Commands,
    mut mapping: ResMut<NetworkObjectMappingServer>,
    mut server: ResMut<RenetServer>,
    mut removed: RemovedComponents<NetworkObject>,
) {
    for id in removed.iter() {
        mapping.remove_obj_and_notify(RemoveNetworkObjectEnum::Entity(id), &mut server);
        commands.entity(id).despawn_recursive();
    }
}
