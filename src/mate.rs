use bevy::prelude::*;

pub mod animation;
pub mod drag;
pub mod window;

use animation::MateAnimationPlugin;
use drag::MateDragPlugin;
use window::MateMoveWindowPlugin;

pub struct MatePlugin;

impl Plugin for MatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MateAnimationPlugin, MateDragPlugin, MateMoveWindowPlugin));
    }
}
