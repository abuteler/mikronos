pub mod components;
pub mod resources;
mod systems;

use systems::layout::spawn_ui;
use resources::TimelineAssets;
use bevy::prelude::*;

pub struct Timeline;

impl Plugin for Timeline {
  fn build(&self, app: &mut App) {
    app.insert_resource(TimelineAssets::default());
    app.add_systems(Startup, spawn_ui);
    app.add_systems(Update, crate::systems::print_chrono_sphere);
  }
}
