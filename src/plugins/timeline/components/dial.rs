use bevy::prelude::*;
use crate::resources::ChronoSphere;
use super::super::systems::timeline_bundles::{
  CONTAINER_PADDING_PX,
  DIAL_WIDTH_PX,
  HOURS_PADDING_PX,
  TIMELINE_WIDTH_PX
};

#[derive(Component)]
pub struct DialMarker;

fn calc_horizontal_position_px(hours:f32, minutes: f32) -> f32 {
  let hour_width_px = (TIMELINE_WIDTH_PX - 23. * 7.) / 24.;
  let cumulative_padding_px = CONTAINER_PADDING_PX + hours * HOURS_PADDING_PX;
  (hours * hour_width_px) + cumulative_padding_px + (hour_width_px/60. * minutes) - DIAL_WIDTH_PX
}

pub fn create_dial_node(hours:f32, minutes: f32) -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Block,
      position_type: PositionType::Absolute,
      top: Val::Px(18.),
      left: Val::Px(calc_horizontal_position_px(hours, minutes)),
      width: Val::Px(DIAL_WIDTH_PX),
      height: Val::Px(130.),
      ..default()
    },
    z_index: ZIndex::Local(1),
    background_color: BackgroundColor(Color::srgba_u8(255, 255, 255, 120)),
    ..default()
  }
}
pub fn update_dial_position(
  chrono: Res<ChronoSphere>,
  mut dial: Query<&mut Style, With<DialMarker>>,
) {
  // Current time
  let new_hh = chrono.hour();
  let new_mm = chrono.minutes();
  let mut style = dial.single_mut();
  style.left = Val::Px(calc_horizontal_position_px(new_hh, new_mm));
}
