use bevy::prelude::*;

use crate::resources::{Fonts, Icons};
use super::layout_bundles::{
  create_app_grid_bundle,
  create_empty_grid_area,
  create_topbar_grid_area,
  create_timeline_header_grid_area,
  create_timeline_body_grid_area,
  create_side_panel_grid_area,
};
use super::topbar::spawn_topbar_contents;
use super::header::spawn_header_contents;
use super::timeline::spawn_timeline_body_contents;

pub fn spawn_ui(mut cmd: Commands, asset_server: Res<AssetServer>, fonts: Res<Fonts>, icons: Res<Icons>) {
  // App container
  let all_father = cmd.spawn(create_app_grid_bundle()).id();
  // Col 1 row 1
  let empty_row = cmd.spawn(create_empty_grid_area()).id();
  // Col 1 row 2
  let topbar = cmd.spawn(create_topbar_grid_area()).id();
  let topbar_contents = spawn_topbar_contents(&mut cmd, &fonts, &icons); // todo: decouple system
  cmd.entity(topbar).push_children(&[topbar_contents]);
  // Col 1 row 3
  let timeline_header = cmd.spawn(create_timeline_header_grid_area()).id();
  let header_contents = spawn_header_contents(&mut cmd, &fonts);
  cmd.entity(timeline_header).push_children(&[header_contents]);
  // Col 1 row 4
  let timeline_body = cmd.spawn(create_timeline_body_grid_area()).id();
  let timeline_body_contents = spawn_timeline_body_contents(&mut cmd, asset_server); // todo: decouple system
  cmd.entity(timeline_body).push_children(&[timeline_body_contents]);

  // Col 2 row 1-4
  let side_modal = cmd.spawn(create_side_panel_grid_area())
    .with_children(|builder| {
      spawn_nested_text_bundle(builder, fonts.medium.clone(), "Side modal", 18.)
    })
    .id();

  cmd.entity(all_father).push_children(&[empty_row,topbar,timeline_header,timeline_body,side_modal]);
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str, font_size: f32) {
  builder.spawn(TextBundle::from_section(
      text,
      TextStyle {
          font,
          font_size,
          color: Color::BLACK,
      },
  ));
}
