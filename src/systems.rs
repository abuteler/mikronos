use bevy::prelude::*;
use crate::resources::ChronoSphere;
use crate::components::CurrentTimeText;

pub fn print_chrono_sphere(
  mut q: Query<&mut Text, With<CurrentTimeText>>,
  chrono: Res<ChronoSphere>
) {
  let mut text = q.single_mut();
  text.sections[0].value = format!("now {:?} x {:?}", chrono.now, chrono.weekday);
}