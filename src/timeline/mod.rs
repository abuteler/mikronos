mod resources;
mod systems;
use systems::load_bg;

use bevy::prelude::*;

pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, load_bg);
  }
}