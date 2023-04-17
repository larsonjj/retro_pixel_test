// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow};
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::io::Cursor;
use winit::window::Icon;

use retro_pixel_test::LibPlugin;
use retro_pixel_test::WINDOW_RESOLUTION_HEIGHT;
use retro_pixel_test::WINDOW_RESOLUTION_WIDTH;

fn main() {
    App::new()
        // Bevy
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.000001, 0.000001, 0.000001)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Retro Pixel Test".to_string(),
                resolution: (WINDOW_RESOLUTION_WIDTH, WINDOW_RESOLUTION_HEIGHT).into(),
                present_mode: PresentMode::Fifo,
                resizable: true,
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        // App Systems
        .add_system(set_window_icon.on_startup())
        // My Plugins
        .add_plugin(LibPlugin)
        .run();
}

// Sets the icon on windows and X11
pub fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
