
use bevy::prelude::*;
use chrono::prelude::*;
use chrono::{DateTime, Local};

// use crate::systems::get_local_now;

#[derive(Resource)]
pub struct ChronoSphere {
  pub now: DateTime<Local>,
}

impl ChronoSphere {
  pub fn new() -> Self {
    let now = Local::now();
    Self {
      now,
    }
  }
  pub fn update_chronosphere(mut chrono: ResMut<ChronoSphere>) {
    chrono.now = Local::now();
  }
  pub fn hour(&self) -> f32 {
    self.now.hour() as f32
  }
  pub fn minutes(&self) -> f32 {
    self.now.minute() as f32
  }
  // For supported formats read: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
  pub fn formatted_hh(&self) -> String  {
    format!("{}", self.now.format("%H"))
  }
  pub fn formatted_mm(&self) -> String  {
    format!("{}", self.now.format("%M"))
  }
  pub fn weekday(&self) -> String {
    format!("{}", self.now.format("%A"))
  }
}

impl Default for ChronoSphere {
  fn default() -> Self {
    Self::new()
  }
}
