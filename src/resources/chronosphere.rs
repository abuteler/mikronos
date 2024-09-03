
use bevy::prelude::*;
// use std::time::Duration;
use time::{OffsetDateTime, Weekday};
use crate::systems::get_local_now;

#[derive(Resource)]
pub struct ChronoSphere {
  pub now: OffsetDateTime,
  pub weekday: Weekday,
  pub hh: u8,
  pub mm: u8,
}

impl ChronoSphere {
  pub fn new() -> Self {
    let now = get_local_now();
    let weekday = now.weekday();
    Self {
      now,
      weekday,
      hh: now.time().hour(),
      mm: now.time().minute(),
    }
  }
}
impl Default for ChronoSphere {
  fn default() -> Self {
    Self::new()
  }
}
