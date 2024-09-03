use bevy::prelude::*;

use crate::resources::{ChronoSphere, Fonts};
use crate::systems::text::create_text_bundle;
use super::components::CurrentTimeText;
use super::header_bundles::create_flex_container_row;

pub fn spawn_header_contents(cmd: &mut Commands, fonts: &Res<Fonts>) -> Entity {
  // Spawn a flex container to insert in the grid area, and hold the topbar elements
  let flex_container_row = cmd.spawn(create_flex_container_row()).id();

  let heading = cmd.spawn((
    create_text_bundle(
      "",
      fonts.medium.clone(),
      18.,
      Color::WHITE,
    ),
    CurrentTimeText
  )).id();

  cmd.entity(flex_container_row).push_children(&[heading]);
  flex_container_row
}

pub fn refresh_heading(
  mut q: Query<&mut Text, With<CurrentTimeText>>,
  chronos: Res<ChronoSphere>
) {
  let mut text = q.single_mut();
  text.sections[0].value = format!("{:?} x {:?}:{:?} {:?}", chronos.weekday, chronos.hh, chronos.mm, chronos.now);
}
