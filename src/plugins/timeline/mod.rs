use bevy::prelude::*;

mod systems;
pub mod components;
pub mod resources;

use systems::layout::spawn_ui;
use systems::header::refresh_heading;
use systems::timeline::refresh_timeline;
use resources::TimelineAssets;
use components::dial::update_dial_position;
// use systems::topbar_buttons::button_system;

pub struct Timeline;

impl Plugin for Timeline {
  fn build(&self, app: &mut App) {
    app.init_resource::<TimelineAssets>();
    app.add_systems(Startup, spawn_ui);
    app.add_systems(Update, refresh_heading);
    app.add_systems(Update, refresh_timeline);
    app.add_systems(Update, update_dial_position);
    // app.add_systems(Update, button_system);
  }
}
