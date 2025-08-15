use bevy::prelude::*;
use bevy::window::WindowPosition;
use bevy::winit::WinitWindows;

#[derive(Event)]
pub struct MoveWindowEvent {
    pub position: IVec2,
}

#[derive(Resource, Debug)]
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
        app.add_event::<MoveWindowEvent>()
            .add_systems(Update, init_screen_bounds.run_if(run_once))
            .add_systems(Update, handle_move_window_events);
    }
}

fn init_screen_bounds(
    mut commands: Commands,
    windows: Query<(Entity, &Window)>,
    winit_windows: Option<NonSend<WinitWindows>>,
) {
    let result = winit_windows.and_then(|winit_windows| {
        windows.single().ok().and_then(|(entity, window)| {
            winit_windows.get_window(entity).map(|winit_window| {
                let (max_x, min_x, max_y, min_y) = winit_window.available_monitors().fold(
                    (i32::MIN, i32::MAX, i32::MIN, i32::MAX),
                    |(max_x, min_x, max_y, min_y), monitor| {
                        let pos = monitor.position();
                        let size = monitor.size();
                        (
                            max_x.max(pos.x + size.width as i32),
                            min_x.min(pos.x),
                            max_y.max(pos.y + size.height as i32),
                            min_y.min(pos.y),
                        )
                    },
                );

                ScreenBounds {
                    min_x,
                    max_x,
                    min_y,
                    max_y,
                    window_width: window.width() as u32,
                    window_height: window.height() as u32,
                }
            })
        })
    });

    match result {
        Some(bounds) => {
            info!("{:#?}", bounds);
            commands.insert_resource(bounds);
        }
        None => warn!("Could not initialize screen bounds, will retry next frame"),
    }
}

fn handle_move_window_events(
    mut events: EventReader<MoveWindowEvent>,
    mut windows: Query<&mut Window>,
    bounds: Option<Res<ScreenBounds>>,
) {
    for event in events.read() {
        if let (Some(bounds), Ok(mut window)) = (bounds.as_ref(), windows.single_mut()) {
            let clamped_x = event.position.x.clamp(
                bounds.min_x,
                bounds.max_x - bounds.window_width as i32,
            );
            let clamped_y = event.position.y.clamp(
                bounds.min_y,
                bounds.max_y - bounds.window_height as i32,
            );

            window.position = WindowPosition::At(IVec2::new(clamped_x, clamped_y));
        }
    }
}

// fn move_window(
//     mut windows: Query<&mut Window>,
//     time: Res<Time>,
//     bounds: Option<Res<ScreenBounds>>,
// ) {
//     if let Some(bounds) = bounds {
//         if let Ok(mut window) = windows.single_mut() {
//             let elapsed = time.elapsed_secs();

//             let range_x = (bounds.max_x - bounds.min_x - bounds.window_width as i32) as f32;
//             let range_y = (bounds.max_y - bounds.min_y - bounds.window_height as i32) as f32;

//             let x = (elapsed * 0.3).sin() * (range_x * 0.4) + (bounds.min_x as f32 + range_x * 0.5);
//             let y = (elapsed * 0.2).cos() * (range_y * 0.3) + (bounds.min_y as f32 + range_y * 0.5);

//             let clamped_x = x.clamp(
//                 bounds.min_x as f32,
//                 (bounds.max_x - bounds.window_width as i32) as f32,
//             );
//             let clamped_y = y.clamp(
//                 bounds.min_y as f32,
//                 (bounds.max_y - bounds.window_height as i32) as f32,
//             );

//             window.position = WindowPosition::At(IVec2::new(clamped_x as i32, clamped_y as i32));
//         }
//     }
// }
