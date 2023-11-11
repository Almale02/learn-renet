use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer};

use crate::{ClientMsg, PlayerEntities, PlayerId};

pub fn hop_player(
    mut q_impulse: Query<&mut ExternalImpulse>,
    player_entity: Res<PlayerEntities>,
    mut server: ResMut<RenetServer>,
) {
    for client_id in server.clients_id().iter() {
        while let Some(req) = server.receive_message(*client_id, DefaultChannel::ReliableUnordered)
        {
            let req: ClientMsg = serde_cbor::from_slice(&req).unwrap();

            match req {
                ClientMsg::Hop(vec) => {
                    q_impulse
                        .get_mut(
                            player_entity
                                .player_id_to_entity
                                .get(&PlayerId(*client_id))
                                .unwrap()
                                .0,
                        )
                        .unwrap()
                        .impulse += vec;
                }
            }
        }
    }
}
