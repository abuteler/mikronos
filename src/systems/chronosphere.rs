use bevy::prelude::*;
use chrono::{DateTime, Local};

use crate::resources::ChronoSphere;

pub fn get_local_now() -> DateTime<Local> {
  Local::now()
}

pub fn update_chronosphere(mut chrono: ResMut<ChronoSphere>) {
  chrono.now = get_local_now();
}
