use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig, DefaultChannel, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use learn_renet::{
    client::{
        handle_inbound_request::{handle_request_client_ordered, handle_request_client_unreliable},
        setup_client::setup_client,
    },
    network_object::NetworkObjectMappingClient,
    utils::{mouse::cursor_to_global_pos, newtype::network_sprite::NetworkSprite},
    *,
};
use local_ip_address::local_ip;

pub fn main() {
    let (client, transport) = create_renet_client();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "logion".into(),
                        resolution: (1000., 600.).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .insert_resource(client)
        .insert_resource(transport)
        .init_resource::<NetworkObjectMappingClient>()
        .add_systems(Startup, setup_client)
        .add_systems(
            Update,
            (
                update_client,
                send_hop,
                handle_request_client_ordered,
                handle_request_client_unreliable,
            ),
        )
        .run();
}

fn create_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let args = std::env::args().collect::<Vec<String>>();
    let mut addr = local_ip().unwrap();
    if args.len() > 1 {
        let ip_arg = args[1].split(".").into_iter().collect::<Vec<&str>>();
        let mut u8_ip_arg: [u8; 4] = [0; 4];

        for (i, val) in ip_arg.iter().enumerate() {
            u8_ip_arg[i] = val.to_string().parse::<u8>().unwrap();
        }
        addr = IpAddr::V4(Ipv4Addr::new(
            u8_ip_arg[0],
            u8_ip_arg[1],
            u8_ip_arg[2],
            u8_ip_arg[3],
        ));
    }

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let client_id = current_time.as_millis() as u64;
    let connection_config = ConnectionConfig::default();
    let server_addr = SocketAddr::new(addr, 42069);

    let authentication = ClientAuthentication::Unsecure {
        protocol_id: PROTOCOL_ID,
        client_id,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    (RenetClient::new(connection_config), transport)
}
fn update_client(_client: Res<RenetClient>) {}
fn send_hop(
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut client: ResMut<RenetClient>,
    key_code: Res<Input<KeyCode>>,
) {
    let Some(mouse_pos) = cursor_to_global_pos(&window, &camera) else {
        return;
    };
    let mouse_pos = mouse_pos.translation;

    if key_code.just_pressed(KeyCode::Space) {
        client.send_message(
            DefaultChannel::ReliableUnordered,
            serde_cbor::to_vec(&ClientMsg::Hop(Vec2::new(mouse_pos.x, mouse_pos.y))).unwrap(),
        );
    }
}
