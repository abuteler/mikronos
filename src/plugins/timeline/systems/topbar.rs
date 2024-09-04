use bevy::prelude::*;

use crate::resources::{Fonts, Icons};
use crate::systems::text::create_text_bundle;
use super::topbar_bundles::{
  create_flex_container_row,
  create_flex_container_left,
  create_flex_container_right,
};

pub fn spawn_topbar_contents(cmd: &mut Commands, fonts: &Res<Fonts>, icons: &Res<Icons>) -> Entity {
  // Spawn a flex container to insert in the grid area, and hold the topbar elements
  let flex_container_row = cmd.spawn(create_flex_container_row()).id();

  // Left container: App icon + title
  let flex_container_left = cmd.spawn(create_flex_container_left()).id();
  let app_icon_bundle = ImageBundle {
    style: Style {
      width: Val::Px(24.),
      height: Val::Px(24.),
      ..default()
    },
    image: UiImage {
      texture: icons.app_icon.clone(),
      ..default()
    },
    ..default()
  };
  let app_icon = cmd.spawn(app_icon_bundle).id();
  let app_title = cmd.spawn(
    create_text_bundle(
      "Timehold",
      fonts.bold.clone(),
      21.,
      Color::BLACK,
    )
  ).id();

  // Right container: icon buttons
  let flex_container_right = cmd.spawn(create_flex_container_right()).id();
  // close icon
  let close_img_bundle = ImageBundle {
    style: Style {
      width: Val::Px(24.),
      height: Val::Px(24.),
      ..default()
    },
    image: UiImage {
      texture: icons.close.clone(),
      ..default()
    },
    ..default()
  };
  let close_icon = cmd.spawn(close_img_bundle).id();
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

  cmd.entity(flex_container_left).push_children(&[app_icon, app_title]);
  cmd.entity(flex_container_right).push_children(&[collapse_icon, close_icon]);
  cmd.entity(flex_container_row).push_children(&[flex_container_left, flex_container_right]);
  flex_container_row
}
