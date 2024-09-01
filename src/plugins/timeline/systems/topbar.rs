use bevy::prelude::*;

use crate::resources::{Fonts, Icons};
use super::topbar_bundles::{
  create_row_container,
  create_title,
};

pub fn spawn_topbar_contents(cmd: &mut Commands, fonts: &Res<Fonts>, icons: &Res<Icons>) -> Entity {
  // Spawn a flex container to insert in the grid area, and hold the topbar elements
  let flex_row_container = cmd.spawn(create_row_container()).id();

  let title = cmd.spawn(create_title("Micronos", fonts.bold.clone())).id();

  // collapse icon
  let collapse_img_bundle = ImageBundle {
    style: Style {
      width: Val::Px(24.),
      height: Val::Px(24.),
      ..default()
    },
    image: UiImage {
      texture: icons.collapse.clone(),
      ..default()
    },
    ..default()
  };
  let collapse_icon = cmd.spawn(collapse_img_bundle).id();

  cmd.entity(flex_row_container).push_children(&[title, collapse_icon]);
  flex_row_container
}
