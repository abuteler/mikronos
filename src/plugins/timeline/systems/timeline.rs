use bevy::prelude::*;

use crate::resources::ChronoSphere;
use crate::systems::get_chronosphere_hh;

use super::components::{Active, CurrentTimeText, Hour};
use super::timeline_bundles::{
  create_col_container,
  create_hours_row_container,
  create_hour_outer,
  create_hour_inner,
};

pub fn spawn_timeline_body_contents(cmd: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
  // Spawn a flex container to insert in the grid area, and hold the timeline elements
  let flex_col_container = cmd.spawn(create_col_container()).id();

  // Timeline background image
  let sprite_handle = asset_server.load("SpectrumBg.png"); // 580 x 140 px
  let background_img_bundle = ImageBundle {
    style: Style {
      width: Val::Px(580.),
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
  info!("time is {}", "potential".to_string());
  let hours_container = cmd.spawn(create_hours_row_container()).id();
  for hour in Hour::VALUES {
    let hour_outer = cmd.spawn(create_hour_outer()).id();
    let hour_inner = cmd.spawn((create_hour_inner(), hour)).id();
    // TODO: load fonts once, centrally?
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let hour_text = cmd.spawn(TextBundle::from_section(
      hour.to_string(),
      TextStyle {
        font,
        font_size: 18.,
        color: Color::WHITE,
      },
    )).id();
    cmd.entity(hour_inner).push_children(&[hour_text]);
    cmd.entity(hour_outer).push_children(&[hour_inner]);
    cmd.entity(hours_container).push_children(&[hour_outer]);
  }

  cmd.entity(flex_col_container).push_children(&[background_img, hours_container]);
  flex_col_container
}

pub fn update_active_hour(
  // mut cmd: Commands,
  hours: Query<(Entity, &Hour)>,
  chronos: Res<ChronoSphere>
) {
  // Current time?
  let hh = get_chronosphere_hh(chronos);
  // for hour in &hours {
  for (_ent, hour) in hours.iter() {
    if hh.to_string().eq(&hour.to_string()) {
      // cmd.entity(hour).insert(Active);
      info!("{} is {}", hh.to_string(), "active".to_string());
    }
  }
}
