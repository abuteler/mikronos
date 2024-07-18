use bevy::prelude::*;
use crate::resources::ChronoSphere;
use crate::plugins::timeline::components::CurrentTimeText;

pub fn print_chrono_sphere(
  mut q: Query<&mut Text, With<CurrentTimeText>>,
  chrono: Res<ChronoSphere>
) {
  let mut text = q.single_mut();
  text.sections[0].value = format!("now {:?} x {:?}", chrono.now, chrono.weekday);
}


fn toggle_window_decorations(mut window: Query<&mut Window>) {
  window.single_mut().decorations = !window.single_mut().decorations;
}
