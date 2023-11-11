use bevy::prelude::*;
pub fn setup_client(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 500.),
        ..default()
    });
}
