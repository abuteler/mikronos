use bevy::prelude::*;

use super::topbar_bundles::{
  create_row_container,
  create_title,
};

pub fn spawn_topbar_contents(cmd: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
  // Spawn a flex container to insert in the grid area, and hold the topbar elements
  let flex_row_container = cmd.spawn(create_row_container()).id();

  // TODO: load fonts once, centrally?
  let font = asset_server.load("fonts/FiraMono-Medium.ttf");

  let title = cmd.spawn(create_title("Micronos", font.clone())).id();

  // collapse icon
  let sprite_handle = asset_server.load("icons/collapse_content_24dp_000000_FILL0_wght400_GRAD0_opsz24.png"); // 580 x 140 px
  let collapse_img_bundle = ImageBundle {
    style: Style {
      width: Val::Px(24.),
      height: Val::Px(24.),
      ..default()
    },
    image: UiImage {
      texture: sprite_handle.clone(),
      ..default()
    },
    ..default()
  };
  let collapse_icon = cmd.spawn(collapse_img_bundle).id();

  cmd.entity(flex_row_container).push_children(&[title, collapse_icon]);
  flex_row_container
}
