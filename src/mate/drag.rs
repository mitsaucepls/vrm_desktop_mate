use bevy::{animation::RepeatAnimation, prelude::*, window::WindowPosition};

use crate::mate::animation::PlayAnimationEvent;
use crate::mate::window::MoveWindowEvent;

const PROGRESS_BAR_SIZE: f32 = 30.0;
const PROGRESS_BAR_RADIUS: f32 = PROGRESS_BAR_SIZE / 2.0;
const PROGRESS_BAR_MARGIN: f32 = 20.0;
const PROGRESS_BAR_BORDER_WIDTH: f32 = 2.0;
const DRAG_TIMER_DURATION: f32 = 1.5;
const PROGRESS_BAR_COLOR: (f32, f32, f32, f32) = (0.8, 0.7, 0.6, 1.0);

pub struct MateDragPlugin;

impl Plugin for MateDragPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DragTimer {
            timer: Timer::from_seconds(DRAG_TIMER_DURATION, TimerMode::Once),
        })
        .insert_resource(CurrentDragState {
            state: DragState::NotHolding,
        })
        .add_systems(Startup, setup_progress_ui)
        .add_systems(Update, (detect_drag, update_progress_ui));
    }
}

#[derive(Debug, PartialEq, Default)]
enum DragState {
    #[default]
    NotHolding,
    Holding,
    ReadyToDrag,
    Dragging {
        initial_mouse_pos: Vec2,
        initial_window_pos: IVec2,
    },
}

#[derive(Component)]
struct ProgressBarContainer;

#[derive(Component)]
struct ProgressBarFill;

#[derive(Resource)]
struct DragTimer {
    timer: Timer,
}

#[derive(Resource)]
struct CurrentDragState {
    state: DragState,
}

fn detect_drag(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    time: Res<Time>,
    mut drag_timer: ResMut<DragTimer>,
    mut drag_state: ResMut<CurrentDragState>,
    mut animation_event_writer: EventWriter<PlayAnimationEvent>,
    mut move_window_event_writer: EventWriter<MoveWindowEvent>,
) {
    let current_mouse_pos = windows.single()
        .ok()
        .and_then(|window| window.cursor_position())
        .unwrap_or(Vec2::ZERO);

    if mouse_input.just_pressed(MouseButton::Left) {
        drag_state.state = DragState::Holding;
        drag_timer.timer.reset();
        println!("Started holding - timer reset");
    }

    if mouse_input.pressed(MouseButton::Left) && drag_state.state == DragState::Holding {
        drag_timer.timer.tick(time.delta());
        println!("Holding... {:.1}s", drag_timer.timer.elapsed_secs());

        if drag_timer.timer.finished() {
            drag_state.state = DragState::ReadyToDrag;
            println!("Ready to drag!");
            animation_event_writer.write(PlayAnimationEvent {
                animation_name: "vrma/VRMA_02.vrma".to_string(),
                repeat: RepeatAnimation::Count(1),
            });
        }
    }

    if mouse_input.pressed(MouseButton::Left) && drag_state.state == DragState::ReadyToDrag {
        let window_pos = windows.single().ok()
            .and_then(|window| {
                if let WindowPosition::At(pos) = window.position {
                    Some(pos)
                } else {
                    None
                }
            })
            .unwrap_or(IVec2::ZERO);

        drag_state.state = DragState::Dragging {
            initial_mouse_pos: current_mouse_pos,
            initial_window_pos: window_pos,
        };
        println!("Started dragging!");
    }

    if mouse_input.pressed(MouseButton::Left) {
        if let DragState::Dragging { initial_mouse_pos, initial_window_pos } = drag_state.state {
            let mouse_delta = current_mouse_pos - initial_mouse_pos;
            let new_window_pos = initial_window_pos + IVec2::new(mouse_delta.x as i32, mouse_delta.y as i32);

            move_window_event_writer.write(MoveWindowEvent {
                position: new_window_pos,
            });
        }
    }

    if mouse_input.just_released(MouseButton::Left) {
        drag_state.state = DragState::NotHolding;
        drag_timer.timer.reset();
        println!("Released - timer reset");
    }
}

fn setup_progress_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(PROGRESS_BAR_SIZE),
                        height: Val::Px(PROGRESS_BAR_SIZE),
                        position_type: PositionType::Absolute,
                        top: Val::Px(PROGRESS_BAR_MARGIN),
                        right: Val::Px(PROGRESS_BAR_MARGIN),
                        border: UiRect::all(Val::Px(PROGRESS_BAR_BORDER_WIDTH)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(PROGRESS_BAR_RADIUS)),
                    BorderColor(Color::srgba(
                        PROGRESS_BAR_COLOR.0,
                        PROGRESS_BAR_COLOR.1,
                        PROGRESS_BAR_COLOR.2,
                        PROGRESS_BAR_COLOR.3,
                    )),
                    BackgroundColor(Color::NONE),
                    Visibility::Hidden,
                    ProgressBarContainer,
                ))
                .with_children(|container| {
                    container.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(PROGRESS_BAR_RADIUS)),
                        BackgroundColor(Color::srgba(
                            PROGRESS_BAR_COLOR.0,
                            PROGRESS_BAR_COLOR.1,
                            PROGRESS_BAR_COLOR.2,
                            PROGRESS_BAR_COLOR.3,
                        )),
                        ProgressBarFill,
                    ));
                });
        });
}

fn update_progress_ui(
    drag_state: Res<CurrentDragState>,
    progress_timer: Res<DragTimer>,
    mut progress_container_query: Query<
        &mut Visibility,
        (With<ProgressBarContainer>, Without<ProgressBarFill>),
    >,
    mut progress_fill_query: Query<&mut BackgroundColor, With<ProgressBarFill>>,
) {
    let Ok(mut visibility) = progress_container_query.single_mut() else {
        return;
    };

    match drag_state.state {
        DragState::NotHolding => {
            *visibility = Visibility::Hidden;
        }
        DragState::Holding | DragState::ReadyToDrag | DragState::Dragging {..} => {
            *visibility = Visibility::Visible;

            if let Ok(mut bg_color) = progress_fill_query.single_mut() {
                let progress = progress_timer.timer.fraction();
                *bg_color = BackgroundColor(Color::srgba(
                    PROGRESS_BAR_COLOR.0,
                    PROGRESS_BAR_COLOR.1,
                    PROGRESS_BAR_COLOR.2,
                    progress,
                ));
            }
        }
    }
}
