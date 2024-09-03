pub mod components;
pub mod resources;
mod systems;

use systems::layout::spawn_ui;
use systems::header::refresh_heading;
use systems::timeline::update_active_hour;
use resources::TimelineAssets;
use bevy::prelude::*;

pub struct Timeline;

impl Plugin for Timeline {
  fn build(&self, app: &mut App) {
    app.insert_resource(TimelineAssets::default());
    app.add_systems(Startup, spawn_ui);
    app.add_systems(Update, refresh_heading);
    app.add_systems(Update, update_active_hour);
  }
}
