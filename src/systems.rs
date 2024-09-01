use bevy::prelude::*;
use time::{OffsetDateTime, UtcOffset};
use crate::resources::ChronoSphere;
use crate::plugins::timeline::components::CurrentTimeText;

pub fn get_local_now() -> OffsetDateTime {
  match OffsetDateTime::now_local() {
    Ok(val) => val,
    Err(_) => OffsetDateTime::now_utc().to_offset(
      UtcOffset::from_hms(-3, 0, 0).expect(
          "IndeterminateOffset for `now_local()`, plus manual setting of offset did not work."
      )
    ),
  }
}

pub fn get_chrono_sphere_hh(chronos: Res<ChronoSphere>) -> u8 {
  chronos.hh
}

pub fn get_chrono_sphere_mm(chronos: Res<ChronoSphere>) -> u8 {
  chronos.mm
}

pub fn update_chrono_sphere(mut chronos: ResMut<ChronoSphere>) {
  let now = get_local_now();
  chronos.now = now;
  chronos.weekday = now.weekday().to_string();
  chronos.hh = now.time().hour();
  chronos.mm = now.time().minute();
}

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
