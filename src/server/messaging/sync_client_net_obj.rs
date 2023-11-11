use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer};

use crate::{network_object::NetworkObjectId, NetworkSprite, ServerMsg};

pub fn sync_client_net_obj(
    q_transform: Query<(Entity, &Transform), Changed<Transform>>,
    q_sprite: Query<&Sprite>,
    q_net_obj: Query<&NetworkObjectId>,
    mut server: ResMut<RenetServer>,
) {
    for (id, transform) in q_transform.iter() {
        let Ok(sprite) = q_sprite.get(id) else {
            return;
        };
        let Ok(net_obj) = q_net_obj.get(id) else {
            return;
        };

        let message = serde_cbor::to_vec(&ServerMsg::SyncNetworkObject(
            net_obj.clone(),
            *transform,
            sprite.clone().into(),
        ))
        .unwrap();

        server.broadcast_message(DefaultChannel::Unreliable, message);
    }
}
