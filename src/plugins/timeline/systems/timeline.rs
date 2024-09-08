use bevy::prelude::*;

use crate::resources::{ChronoSphere, Fonts};
use crate::systems::create_text_bundle;

use super::components::{Active, Hour, DialMarker};
use super::timeline_bundles::{
  create_flex_container_col,
  create_hours_row_container,
  create_hour_outer,
  create_hour_inner,
  create_dial,
  CONTAINER_PADDING_PX,
  TIMELINE_WIDTH_PX,
  HOURS_PADDING_PX,
  DIAL_WIDTH_PX,
};

pub fn spawn_timeline_body_contents(cmd: &mut Commands, asset_server: Res<AssetServer>, fonts: &Res<Fonts>, chrono: Res<ChronoSphere>) -> Entity {
  // Spawn a flex container to insert in the grid area, and hold the timeline elements
  let body_container = cmd.spawn(create_flex_container_col()).id();

  // Timeline background image
  let sprite_handle = asset_server.load("SpectrumBg.png"); // 580 x 140 px
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
  // Add dial
  let position = calc_dial_horizontal_position(chrono.hour(), chrono.minutes());
  let dial = cmd.spawn((create_dial(position), DialMarker)).id();

  cmd.entity(body_container).push_children(&[background_img, hours_container, dial]);
  body_container
}

pub fn refresh_timeline(
  chrono: Res<ChronoSphere>,
  mut dial: Query<&mut Style, With<DialMarker>>,
  // mut cmd: Commands,
  hours: Query<(Entity, &Hour)>,
) {
  // Current time
  let new_hh = chrono.hour();
  let new_mm = chrono.minutes();
  let mut style = dial.single_mut();
  style.left = Val::Px(calc_dial_horizontal_position(new_hh, new_mm));

  // for hour in &hours {
  for (_ent, hour) in hours.iter() {
    let formatted_new_hh = chrono.formatted_hh();
    if formatted_new_hh.eq(&hour.to_string()) {
      // cmd.entity(hour).insert(Active);
      // info!("Now is {}:{}", formatted_new_hh, chrono.formatted_mm());
    }
  }
}

fn calc_dial_horizontal_position(hours:f32, minutes: f32) -> f32 {
  let hour_width_px = (TIMELINE_WIDTH_PX - 23. * 7.) / 24.;
  let cumulative_padding_px = CONTAINER_PADDING_PX + hours * HOURS_PADDING_PX;
  (hours * hour_width_px) + cumulative_padding_px + (hour_width_px/60. * minutes) - DIAL_WIDTH_PX
}
