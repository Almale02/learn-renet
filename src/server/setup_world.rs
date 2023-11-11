use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::{
    network_object::NetworkObjectMappingServer, DefaultEntity, NetworkObject, PlayerEntities,
    PlayerId,
};

pub fn setup_scene(mut commands: Commands) {
    let width = 999.;
    let height = 333.;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 500.),
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(width, 15.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -70., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(width / 2., 7.5),
        NetworkObject,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(width, 15.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -70. + height, 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(width / 2., 7.5),
        NetworkObject,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(15., height)),
                ..default()
            },
            transform: Transform::from_xyz(width / 2., -70. + height / 2., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(7.5, height / 2.),
        NetworkObject,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(15., height)),
                ..default()
            },
            transform: Transform::from_xyz(-width / 2., -70. + height / 2., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(7.5, height / 2.),
        NetworkObject,
    ));
    for _ in 1..20 {
        /*commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(3., 3.)),
                    ..default()
                },
                transform: Transform::default(),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(1.5, 1.5),
            NetworkObject,
        ));*/
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(15., height)),
                ..default()
            },
            transform: Transform::from_xyz(-width / 2., -70. + height / 2., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(7.5, height / 2.),
        NetworkObject,
    ));
}
pub fn spawn_player(
    commands: &mut Commands,
    player_entities: &mut ResMut<PlayerEntities>,
    user_id: PlayerId,
) -> Entity {
    let id = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(30., 30.)),
                    ..default()
                },
                transform: Transform::from_xyz(1., 1., 1.),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(15., 15.),
            ExternalImpulse::default(),
            ExternalForce::default(),
            //Velocity::default(),
            NetworkObject,
        ))
        .id();
    player_entities
        .player_id_to_entity
        .insert(user_id, DefaultEntity(id));
    /*let net_id = net_obj_mapping.add_obj_and_notify(id, server).unwrap();
    commands.entity(id).insert(net_id);*/
    return id;
}
