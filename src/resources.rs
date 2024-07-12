use bevy::prelude::*;
// use std::time::Duration;
use time::{OffsetDateTime, UtcOffset};

#[derive(Resource)]
pub struct ChronoSphere {
  pub now: OffsetDateTime,
  pub weekday: String,
}

impl ChronoSphere {
  pub fn new() -> Self {
    let now = match OffsetDateTime::now_local() {
      Ok(val) => val,
      Err(_) => OffsetDateTime::now_utc().to_offset(
          UtcOffset::from_hms(-3, 0, 0).expect(
              "IndeterminateOffset for `now_local()`, plus manual setting of offset did not work."
          )
      ),
    };
    let weekday = now.weekday().to_string();
    Self {
      now,
      weekday,
    }
  }
}
impl Default for ChronoSphere {
  fn default() -> Self {
    Self::new()
  }
}