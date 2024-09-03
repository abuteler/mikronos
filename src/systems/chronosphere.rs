use bevy::prelude::*;
use time::{OffsetDateTime, UtcOffset, Weekday};
use crate::resources::ChronoSphere;

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

pub fn get_chronosphere_hh(chronos: Res<ChronoSphere>) -> u8 {
  chronos.hh
}

pub fn get_chronosphere_mm(chronos: Res<ChronoSphere>) -> u8 {
  chronos.mm
}

pub fn get_chronosphere_weekday(chronos: Res<ChronoSphere>) -> Weekday {
  chronos.weekday
}

pub fn update_chronosphere(mut chronos: ResMut<ChronoSphere>) {
  let now = get_local_now();
  chronos.now = now;
  chronos.weekday = now.weekday();
  chronos.hh = now.time().hour();
  chronos.mm = now.time().minute();
}
