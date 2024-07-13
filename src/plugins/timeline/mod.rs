mod resources;
mod systems;
use systems::load_bg;

use bevy::prelude::*;

pub struct Timeline;

impl Plugin for Timeline {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, load_bg);
  }
}