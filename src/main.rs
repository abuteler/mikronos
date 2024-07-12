use bevy::{
    prelude::*,
    window::{
        WindowResized, WindowResolution, WindowTheme, EnabledButtons
    },
    winit::WinitSettings,
};
use micronos::{resources::ChronoSphere, timeline::TimelinePlugin};

// MEMO: ECS (Entity Component System)
    // Entities are ids u32 (+ generation #)
    // Components are Struct, Enum "things that are stateful"
    //     Resources are components without an entity attached.
    // Systems are functions
    // Great to watch: https://youtu.be/CnoDOc6ML0Y?si=EEL5Mvo0FYfPoonw&t=957

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "micronos".into(),
        name: Some("micronos".into()),
        resolution: WindowResolution::new(600., 208.).with_scale_factor_override(1.0),
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
    .add_plugins(TimelinePlugin)
    .add_systems(Startup, (setup_camera, setup_ui))
    .add_systems(Update, on_resize_system)
    .add_systems(Update, print_chrono_sphere)
    .run();
}

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;

#[derive(Component)]
struct CurrentTimeText;

// Spawns the camera that draws UI
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

// Spawns the UI
fn setup_ui(mut cmd: Commands) {
  // Node that fills entire background
  cmd.spawn(NodeBundle {
      style: Style {
          width: Val::Percent(100.),
          ..default()
      },
      ..default()
  })
  .with_children(|root| {
      // Text where we display current resolution
      root.spawn((
          TextBundle::from_section(
              "Resolution",
              TextStyle {
                  font_size: 50.0,
                  ..default()
              },
          ),
          ResolutionText,
      ));
      root.spawn((
          TextBundle::from_section(
              "CurrentTime",
              TextStyle {
                  font_size: 50.0,
                  ..default()
              },
          ),
          CurrentTimeText,
      ));
    });
}

fn toggle_window_decorations(mut window: Query<&mut Window>) {
  window.single_mut().decorations = !window.single_mut().decorations;
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.read() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}

fn print_chrono_sphere(
    mut q: Query<&mut Text, With<CurrentTimeText>>,
    chrono: Res<ChronoSphere>
) {
    let mut text = q.single_mut();
    text.sections[0].value = format!("now {:?} x {:.1}", chrono.now, chrono.weekday);
}