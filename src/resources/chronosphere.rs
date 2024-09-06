
use bevy::prelude::*;

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
  pub fn get_chronosphere_hh(&self) -> f32 {
    self.hh as f32
  }
  pub fn get_chronosphere_mm(&self) -> f32 {
    self.mm as f32
  }
  pub fn get_chronosphere_weekday(&self) -> Weekday {
    self.weekday
  }
}

impl Default for ChronoSphere {
  fn default() -> Self {
    Self::new()
  }
}
