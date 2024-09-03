// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    prelude::*,
    window::{
        EnabledButtons, WindowLevel, WindowResolution, WindowTheme
    },
    winit::WinitSettings,
};
use timehold::{
  plugins::Timeline,
  resources::{ChronoSphere, Fonts, Icons},
  systems::{update_chronosphere, set_window_icon},
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "timehold".into(),
        name: Some("timehold".into()),
        resolution: WindowResolution::new(850., 450.).with_scale_factor_override(1.0),
        window_theme: Some(WindowTheme::Dark),
        decorations: false,
        transparent: true,
        window_level: WindowLevel::AlwaysOnTop,
        enabled_buttons: EnabledButtons {
          minimize: false,
          maximize: false,
          close: false,
        },
        ..default()
      }),
      ..default()
    }))
    // Reduce CPU/GPU use when app is unfocused // TODO: chequear que no reconstruya todo en cada loop
    .insert_resource(WinitSettings::desktop_app())

    // ClearColor must have 0 alpha, otherwise some color will bleed through
    .insert_resource(ClearColor(Color::NONE))
    .init_resource::<ChronoSphere>()
    .init_resource::<Fonts>()
    .init_resource::<Icons>()
    .add_plugins(Timeline)
    .add_systems(Startup, set_window_icon)
    .add_systems(Startup, setup_camera)
    .add_systems(Update, update_chronosphere)
    .run();
}

// Spawns the camera that draws UI
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
