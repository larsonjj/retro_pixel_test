// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::camera::ScalingMode;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};

pub const WINDOW_RESOLUTION_WIDTH: f32 = 960.;
pub const WINDOW_RESOLUTION_HEIGHT: f32 = 576.;
pub const REFERENCE_RESOLUTION_WIDTH: f32 = 320.;
pub const REFERENCE_RESOLUTION_HEIGHT: f32 = 192.;

pub struct LibPlugin;

impl Plugin for LibPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<CustomMaterial>::default())
            .add_system(setup_camera.on_startup())
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

#[derive(AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[uniform(0, CustomMaterialUniform)]
#[reflect(Default, Debug)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    color: Color,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    texture: Option<Handle<Image>>,
}

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        // "shaders/color_texture.wgsl".into()
        "shaders/smooth_pixel.wgsl".into()
    }
}

impl Default for CustomMaterial {
    fn default() -> Self {
        CustomMaterial {
            color: Color::WHITE,
            texture: None,
        }
    }
}

impl From<Color> for CustomMaterial {
    fn from(color: Color) -> Self {
        CustomMaterial {
            color,
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for CustomMaterial {
    fn from(texture: Handle<Image>) -> Self {
        CustomMaterial {
            texture: Some(texture),
            ..Default::default()
        }
    }
}

/// The GPU representation of the uniform data of a [`CustomMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct CustomMaterialUniform {
    pub color: Vec4,
    pub flags: u32,
}

impl AsBindGroupShaderType<CustomMaterialUniform> for CustomMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> CustomMaterialUniform {
        let mut flags = CustomMaterialUniformFlags::NONE;
        if self.texture.is_some() {
            flags |= CustomMaterialUniformFlags::TEXTURE;
        }

        CustomMaterialUniform {
            color: self.color.as_linear_rgba_f32().into(),
            flags: flags.bits(),
        }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct CustomMaterialUniformFlags: u32 {
        const TEXTURE           = (1 << 0);
        const NONE              = 0;
        const UNINITIALIZED     = 0xFFFF;
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

// Spawn an entity using `CustomMaterial`.
fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_assets
                .add(Mesh::from(shape::Quad {
                    size: Vec2 { x: 16.0, y: 16.0 },
                    ..default()
                }))
                .into(),
            material: materials.add(CustomMaterial {
                color: Color::RED,
                texture: Some(asset_server.load("textures/player_proto.png")),
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Player,
    ));

    // commands.spawn((
    //     PixelSpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(20.0, 0.0, 0.0),
    //             scale: Vec3::new(16.0, 16.0, 0.0),
    //             rotation: Quat::IDENTITY,
    //         },
    //         material: materials.add(CustomMaterial {
    //             color: Color::RED,
    //             texture: Some(asset_server.load("textures/enemy_proto.png")),
    //         }),
    //         ..Default::default()
    //     },
    //     Enemy,
    // ));

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
        // player_controller.rotation = Quat::from_rotation_y(75.0);
    }
}
