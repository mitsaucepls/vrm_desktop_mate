use bevy::prelude::*;
use bevy::window::{WindowMode, WindowPosition};
use bevy::winit::WinitWindows;
use bevy_vrm::{VrmPlugins, VrmBundle, VrmInstance, VrmScene, loader::Vrm};

#[derive(Component)]
struct RotatingVrmModel;

#[derive(Resource)]
struct ScreenBounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    window_width: u32,
    window_height: u32,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
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
        }).set(AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..default()
        }), VrmPlugins))
        .insert_resource(ClearColor(Color::srgba(0.0, 0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_vrm_model, move_window))
        .add_systems(Update, init_screen_bounds_delayed.run_if(run_once))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        VrmBundle {
            scene: VrmScene::default(),
            vrm: VrmInstance(asset_server.load("model.vrm")),
            ..default()
        },
        RotatingVrmModel,
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 2000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 3.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}

fn rotate_vrm_model(mut query: Query<&mut Transform, With<RotatingVrmModel>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

fn init_screen_bounds_delayed(
    mut commands: Commands, 
    windows: Query<(Entity, &Window)>,
    winit_windows: Option<NonSend<WinitWindows>>
) {
    if let (Ok((entity, window)), Some(winit_windows)) = (windows.single(), winit_windows) {
        let mut min_x = 0i32;
        let mut max_x = 1920i32;
        let mut min_y = 0i32;
        let mut max_y = 1080i32;
        
        if let Some(winit_window) = winit_windows.get_window(entity) {
            for monitor in winit_window.available_monitors() {
                let pos = monitor.position();
                let size = monitor.size();
                
                min_x = min_x.min(pos.x);
                min_y = min_y.min(pos.y);
                max_x = max_x.max(pos.x + size.width as i32);
                max_y = max_y.max(pos.y + size.height as i32);
            }
        }
        
        let bounds = ScreenBounds {
            min_x,
            max_x,
            min_y,
            max_y,
            window_width: window.width() as u32,
            window_height: window.height() as u32,
        };
        
        println!("Screen bounds detected: {}x{} to {}x{}", min_x, min_y, max_x, max_y);
        commands.insert_resource(bounds);
    }
}

fn move_window(
    mut windows: Query<&mut Window>, 
    time: Res<Time>,
    bounds: Option<Res<ScreenBounds>>
) {
    if let Some(bounds) = bounds {
        if let Ok(mut window) = windows.single_mut() {
            let elapsed = time.elapsed_secs();
            
            let range_x = (bounds.max_x - bounds.min_x - bounds.window_width as i32) as f32;
            let range_y = (bounds.max_y - bounds.min_y - bounds.window_height as i32) as f32;
            
            let x = (elapsed * 0.3).sin() * (range_x * 0.4) + (bounds.min_x as f32 + range_x * 0.5);
            let y = (elapsed * 0.2).cos() * (range_y * 0.3) + (bounds.min_y as f32 + range_y * 0.5);
            
            let clamped_x = x.clamp(bounds.min_x as f32, (bounds.max_x - bounds.window_width as i32) as f32);
            let clamped_y = y.clamp(bounds.min_y as f32, (bounds.max_y - bounds.window_height as i32) as f32);
            
            window.position = WindowPosition::At(IVec2::new(clamped_x as i32, clamped_y as i32));
        }
    }
}
