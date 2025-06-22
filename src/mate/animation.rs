use bevy::prelude::*;
use bevy_vrm1::prelude::*;

#[derive(Resource)]
pub struct AnimationCycler {
    pub animations: Vec<String>,
    pub current_index: usize,
    pub vrm_entity: Option<Entity>,
    pub animation_start_time: Option<f32>,
    pub animation_duration: Option<f32>,
}

pub struct MateAnimationPlugin;

impl Plugin for MateAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_animation_cycler)
            .add_systems(Startup, spawn_vrm.after(setup_animation_cycler))
            .add_systems(Update, check_animation_finished);
    }
}

fn setup_animation_cycler(mut commands: Commands) {
    let mut animations = Vec::new();

    if let Ok(entries) = std::fs::read_dir("assets/vrma") {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if extension == "vrma" {
                    if let Some(filename) = entry.path().file_name() {
                        if let Some(name) = filename.to_str() {
                            animations.push(format!("vrma/{}", name));
                        }
                    }
                }
            }
        }
    }

    animations.sort();

    commands.insert_resource(AnimationCycler {
        animations,
        current_index: 0,
        vrm_entity: None,
        animation_start_time: None,
        animation_duration: None,
    });
}

fn spawn_vrm(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cycler: ResMut<AnimationCycler>,
) {
    let vrm_entity = commands
        .spawn(VrmHandle(
            asset_server.load("vrm/HatsuneMikuNTReformed.vrm"),
        ))
        .id();

    cycler.vrm_entity = Some(vrm_entity);

    commands.entity(vrm_entity).with_children(|cmd| {
        cmd.spawn(VrmaHandle(asset_server.load(&cycler.animations[0])))
            .observe(apply_play_vrma);
    });
}

fn apply_play_vrma(
    trigger: Trigger<LoadedVrma>,
    mut commands: Commands,
    mut cycler: ResMut<AnimationCycler>,
    time: Res<Time>,
    vrma_duration_query: Query<&bevy_vrm1::vrma::VrmaDuration>,
) {
    let vrma_entity = trigger.target();

    if let Ok(duration) = vrma_duration_query.get(vrma_entity) {
        cycler.animation_duration = Some(duration.0.as_secs_f32());
        cycler.animation_start_time = Some(time.elapsed_secs());
    }

    commands
        .entity(vrma_entity)
        .trigger(PlayVrma { repeat: false });
}

fn check_animation_finished(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cycler: ResMut<AnimationCycler>,
    children_query: Query<&Children>,
    vrma_handle_query: Query<&VrmaHandle>,
    time: Res<Time>,
) {
    if let Some(vrm_entity) = cycler.vrm_entity {
        if let (Some(start_time), Some(duration)) =
            (cycler.animation_start_time, cycler.animation_duration)
        {
            let elapsed = time.elapsed_secs() - start_time;

            if elapsed >= duration {
                if let Ok(children) = children_query.get(vrm_entity) {
                    for child in children.iter() {
                        if vrma_handle_query.get(child).is_ok() {
                            commands.entity(child).despawn();
                        }
                    }
                }

                cycler.current_index = (cycler.current_index + 1) % cycler.animations.len();

                cycler.animation_start_time = None;
                cycler.animation_duration = None;

                commands.entity(vrm_entity).with_children(|cmd| {
                    cmd.spawn(VrmaHandle(
                        asset_server.load(&cycler.animations[cycler.current_index]),
                    ))
                    .observe(apply_play_vrma);
                });
            }
        }
    }
}
