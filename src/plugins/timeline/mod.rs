use bevy::prelude::*;

mod systems;
pub mod components;
pub mod resources;

use systems::layout::spawn_ui;
use systems::header::refresh_heading;
use systems::timeline::refresh_timeline;
use resources::TimelineAssets;
// use systems::topbar_buttons::button_system;

pub struct Timeline;

impl Plugin for Timeline {
  fn build(&self, app: &mut App) {
    app.insert_resource(TimelineAssets::default());
    app.add_systems(Startup, spawn_ui);
    app.add_systems(Update, refresh_heading);
    app.add_systems(Update, refresh_timeline);
    // app.add_systems(Update, button_system);
  }
}
