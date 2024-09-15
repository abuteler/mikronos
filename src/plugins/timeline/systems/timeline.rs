use bevy::prelude::*;
use crate::plugins::timeline::resources::TimelineAssets;
use crate::resources::{ChronoSphere, Fonts};
use crate::systems::create_text_bundle;

use super::components::{Active, Hour};
use super::components::dial::{DialMarker, create_dial_node};
use super::timeline_bundles::{
  create_flex_container_col,
  create_hours_row_container,
  create_hour_outer,
  create_hour_inner,
  TIMELINE_WIDTH_PX,
};

pub fn spawn_timeline_body_contents(cmd: &mut Commands, timeline_assets: Res<TimelineAssets>, fonts: &Res<Fonts>, chrono: Res<ChronoSphere>) -> Entity {
  // Spawn a flex container to insert in the grid area, and hold the timeline elements
  let body_container = cmd.spawn(create_flex_container_col()).id();

  // Timeline background image
  let sprite_handle = timeline_assets.spectrum_bg.clone(); // 580 x 140 px
  let background_img_bundle = ImageBundle {
    style: Style {
      width: Val::Px(TIMELINE_WIDTH_PX),
      height: Val::Px(140.),
      ..default()
    },
    image: UiImage {
      texture: sprite_handle.clone(),
      ..default()
    },
    ..default()
  };
  let background_img = cmd.spawn(background_img_bundle).id();

  // START Hour boxes
  info!("Time is {}", "potential".to_string());
  let hours_container = cmd.spawn(create_hours_row_container()).id();
  for hour in Hour::VALUES {
    let hour_outer = cmd.spawn(create_hour_outer()).id();
    let hour_inner = cmd.spawn((create_hour_inner(), hour)).id();
    let hour_text = cmd.spawn(create_text_bundle(
      &hour.to_string(),
      fonts.medium.clone(),
      18.,
      Color::WHITE,
    )).id();
    cmd.entity(hour_inner).push_children(&[hour_text]);
    cmd.entity(hour_outer).push_children(&[hour_inner]);
    cmd.entity(hours_container).push_children(&[hour_outer]);
  }
  // Create and insert dial
  let dial = cmd.spawn((
    DialMarker,
    create_dial_node(chrono.hour(), chrono.minutes())
  )).id();

  cmd.entity(body_container).push_children(&[background_img, hours_container, dial]);
  body_container
}

pub fn refresh_timeline(
  chrono: Res<ChronoSphere>,
  // mut cmd: Commands,
  hours: Query<(Entity, &Hour)>,
) {
  // for hour in &hours {
  for (_ent, hour) in hours.iter() {
    let formatted_new_hh = chrono.formatted_hh();
    if formatted_new_hh.eq(&hour.to_string()) {
      // cmd.entity(hour).insert(Active);
      info!("Now is {}:{}", formatted_new_hh, chrono.formatted_mm());
    }
  }
}
