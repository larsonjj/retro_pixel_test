// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const WINDOW_RESOLUTION_WIDTH: f32 = 960.;
pub const WINDOW_RESOLUTION_HEIGHT: f32 = 576.;
pub const REFERENCE_RESOLUTION_WIDTH: f32 = 320.;
pub const REFERENCE_RESOLUTION_HEIGHT: f32 = 192.;

pub struct LibPlugin;

impl Plugin for LibPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_camera.on_startup())
            .add_system(setup_player.on_startup())
            .add_system(move_player);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

pub fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    let largest_reference_dimension = REFERENCE_RESOLUTION_WIDTH.max(REFERENCE_RESOLUTION_HEIGHT);
    let largest_window_dimension = WINDOW_RESOLUTION_WIDTH.max(WINDOW_RESOLUTION_HEIGHT);
    let scale = largest_reference_dimension / largest_window_dimension;

    camera.transform = Transform::from_xyz(0., 0., 1000.0);
    camera.projection.scaling_mode = ScalingMode::FixedVertical(WINDOW_RESOLUTION_WIDTH);
    camera.projection.scale = scale;

    commands.spawn(camera);
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

// Spawn an entity using `CustomMaterial`.
fn setup_player(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_proto.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Player,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_proto.png"),
            transform: Transform::from_xyz(20.0, 0.0, 0.0),
            ..Default::default()
        },
        Enemy,
    ));
}

fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    for mut player_controller in &mut player_query {
        player_controller.translation += Vec3::new(0.2, 0.2, 0.0);
    }
}
