use bevy::ecs::world::EntityMut;
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
};
use learn_renet::{
    network_object::{NetworkObjectMappingServer, RemoveNetworkObjectEnum},
    server::messaging::{
        hop_player::hop_player,
        sync_client_net_obj::sync_client_net_obj,
        sync_spawn_despawn::{sync_despawn_net_obj, sync_spawn_net_obj},
    },
    utils::newtype::network_sprite::NetworkSprite,
    *,
};
use local_ip_address::local_ip;
use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy_renet::RenetServerPlugin;
use learn_renet::server::setup_world::*;

pub fn main() {
    let (server, transport) = create_renet_server();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(30.))
        .insert_resource(server)
        .insert_resource(transport)
        .init_resource::<PlayerEntities>()
        .init_resource::<NetworkObjectMappingServer>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                server_events,
                hop_player,
                sync_client_net_obj,
                sync_spawn_net_obj,
                sync_despawn_net_obj,
            ),
        )
        .run();
}
fn create_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server = RenetServer::new(ConnectionConfig::default());

    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addr: server_addr,
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();
    (server, transport)
}
fn server_events(
    mut commands: Commands,
    q_transform: Query<&Transform>,
    q_sprite: Query<&Sprite>,
    mut player_entities: ResMut<PlayerEntities>,
    net_obj_mapping: ResMut<NetworkObjectMappingServer>,
    mut events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                spawn_player(&mut commands, &mut player_entities, PlayerId(*client_id));
                for (net_obj, entity) in net_obj_mapping.obj_entity.iter() {
                    let message = serde_cbor::to_vec(&ServerMsg::SpawnNetworkObject(
                        *net_obj,
                        q_transform.get(entity.clone()).unwrap().clone(),
                        q_sprite.get(entity.clone()).unwrap().clone().into(),
                    ))
                    .unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableUnordered, message)
                }
            }
            ServerEvent::ClientDisconnected {
                client_id,
                reason: _,
            } => {
                let id = commands
                    .entity(
                        player_entities
                            .player_id_to_entity
                            .remove(&PlayerId(*client_id))
                            .unwrap()
                            .0,
                    )
                    .id();
                commands.entity(id).remove::<NetworkObject>();
            }
        };
    }
}
