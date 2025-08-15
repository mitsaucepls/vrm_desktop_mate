use std::time::Duration;

use bevy::{animation::RepeatAnimation, prelude::*};
use bevy_vrm1::prelude::*;

pub struct MateAnimationPlugin;

impl Plugin for MateAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayAnimationEvent>()
            .add_systems(Startup, spawn_vrm)
            .add_systems(Update, handle_play_animation_events);
    }
}

#[derive(Event)]
pub struct PlayAnimationEvent {
    pub animation_name: String,
    pub repeat: RepeatAnimation,
}

fn handle_play_animation_events(
    mut events: EventReader<PlayAnimationEvent>,
    mut commands: Commands,
    vrma_query: Query<Entity, With<Vrma>>,
    vrm_query: Query<Entity, With<Vrm>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let repeat = event.repeat;

        for vrma_entity in vrma_query.iter() {
            commands.entity(vrma_entity).despawn();
        }

        for vrm_entity in vrm_query.iter() {
            commands.entity(vrm_entity).with_children(|child_command| {
                child_command
                    .spawn(VrmaHandle(asset_server.load(&event.animation_name)))
                    .observe(
                        move |trigger: Trigger<LoadedVrma>, mut commands: Commands| {
                            let vrma_entity = trigger.target();
                            commands.entity(vrma_entity).trigger(PlayVrma {
                                repeat,
                                transition_duration: Duration::ZERO,
                            });
                        },
                    );
            });
        }
    }
}

fn spawn_vrm(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(VrmHandle(
        asset_server.load("vrm/HatsuneMikuNTReformed.vrm"),
    ));
}
