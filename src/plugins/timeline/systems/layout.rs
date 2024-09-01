use bevy::prelude::*;

use crate::resources::ChronoSphere;
use crate::systems::get_chrono_sphere_hh;

use super::components::{Active, CurrentTimeText, Hour};
use super::layout_bundles::{
  create_app_grid_bundle,
  create_empty_grid_area,
  create_topbar_grid_area,
  create_timeline_header_grid_area,
  create_timeline_body_grid_area,
  create_side_panel_grid_area,
};
use super::topbar::spawn_topbar_contents;
use super::timeline::spawn_timeline_body_contents;

pub fn spawn_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  // App container
  let all_father = cmd.spawn(create_app_grid_bundle()).id();
  // Col 1 row 1
  let empty_row = cmd.spawn(create_empty_grid_area()).id();
  // Col 1 row 2
  let topbar = cmd.spawn(create_topbar_grid_area()).id();
  let topbar_contents = spawn_topbar_contents(&mut cmd, &asset_server);
  cmd.entity(topbar).push_children(&[topbar_contents]);
  // Col 1 row 3
  let timeline_header = cmd.spawn(create_timeline_header_grid_area())
    .with_children(|builder| {
      // current time placer
      builder
        .spawn((
          TextBundle::from_section(
            "CurrentTime",
            TextStyle {
              font_size: 14.0,
              ..default()
            },
          ),
          CurrentTimeText,
        ));
      })
    .id();
  // Col 1 row 4
  let timeline_body = cmd.spawn(create_timeline_body_grid_area()).id();
  let timeline_body_contents = spawn_timeline_body_contents(&mut cmd, asset_server);
  cmd.entity(timeline_body).push_children(&[timeline_body_contents]);

  // Col 2 row 1-4
  let side_modal = cmd.spawn(create_side_panel_grid_area())
    .with_children(|builder| {
      spawn_nested_text_bundle(builder, font.clone(), "Side modal", 18.)
    })
    .id();

  cmd.entity(all_father).push_children(&[empty_row,topbar,timeline_header,timeline_body,side_modal]);
}

