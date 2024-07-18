use bevy::{
    prelude::*,
    window::{
        WindowResolution, WindowTheme, EnabledButtons
    },
    winit::WinitSettings,
};
use micronos::{plugins::Timeline, resources::ChronoSphere};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "micronos".into(),
        name: Some("micronos".into()),
        resolution: WindowResolution::new(850., 450.).with_scale_factor_override(1.0),
        window_theme: Some(WindowTheme::Dark),
        decorations: true,
        transparent: true,
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
    .insert_resource(ChronoSphere::new())
    .add_plugins(Timeline)
    .add_systems(Startup, setup_camera)
    // .add_systems(Update, print_chrono_sphere)
    .run();
}

// Spawns the camera that draws UI
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
