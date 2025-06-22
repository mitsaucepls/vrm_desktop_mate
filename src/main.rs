use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_vrm1::prelude::*;

mod mate;
use mate::MatePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        transparent: true,
                        decorations: false,
                        window_level: bevy::window::WindowLevel::AlwaysOnTop,
                        mode: WindowMode::Windowed,
                        resizable: false,
                        focused: false,
                        skip_taskbar: true,
                        window_theme: Some(bevy::window::WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                }),
            VrmPlugin,
            VrmaPlugin,
            MatePlugin,
        ))
        .insert_resource(ClearColor(Color::srgba(0.0, 0.0, 0.0, 0.0)))
        .add_systems(Startup, (spawn_directional_light, spawn_camera))
        .run();
}

fn spawn_directional_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(3.0, 3.0, 0.3).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 1.0, 3.0)));
}
