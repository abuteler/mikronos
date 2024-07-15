mod resources;
mod systems;
use systems::layout::spawn_ui;

use bevy::prelude::*;

pub struct Timeline;

impl Plugin for Timeline {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_ui);
  }
}