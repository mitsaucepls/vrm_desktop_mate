use bevy::prelude::*;
use bevy::window::WindowPosition;
use bevy::winit::WinitWindows;

#[derive(Resource)]
pub struct ScreenBounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub window_width: u32,
    pub window_height: u32,
}

pub struct MateMoveWindowPlugin;

impl Plugin for MateMoveWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_window)
            .add_systems(Update, init_screen_bounds_delayed.run_if(run_once));
    }
}

fn init_screen_bounds_delayed(
    mut commands: Commands,
    windows: Query<(Entity, &Window)>,
    winit_windows: Option<NonSend<WinitWindows>>,
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

        println!(
            "Screen bounds detected: {}x{} to {}x{}",
            min_x, min_y, max_x, max_y
        );
        commands.insert_resource(bounds);
    }
}

fn move_window(
    mut windows: Query<&mut Window>,
    time: Res<Time>,
    bounds: Option<Res<ScreenBounds>>,
) {
    if let Some(bounds) = bounds {
        if let Ok(mut window) = windows.single_mut() {
            let elapsed = time.elapsed_secs();

            let range_x = (bounds.max_x - bounds.min_x - bounds.window_width as i32) as f32;
            let range_y = (bounds.max_y - bounds.min_y - bounds.window_height as i32) as f32;

            let x = (elapsed * 0.3).sin() * (range_x * 0.4) + (bounds.min_x as f32 + range_x * 0.5);
            let y = (elapsed * 0.2).cos() * (range_y * 0.3) + (bounds.min_y as f32 + range_y * 0.5);

            let clamped_x = x.clamp(
                bounds.min_x as f32,
                (bounds.max_x - bounds.window_width as i32) as f32,
            );
            let clamped_y = y.clamp(
                bounds.min_y as f32,
                (bounds.max_y - bounds.window_height as i32) as f32,
            );

            window.position = WindowPosition::At(IVec2::new(clamped_x as i32, clamped_y as i32));
        }
    }
}
