use bevy::prelude::*;

use crate::systems::print_chrono_sphere;

use super::components::{CurrentTimeText, Hour, HourBox};
use super::ui_bundles::{
  create_app_grid_bundle,
  create_empty_grid_area,
  create_top_bar_grid_area,
  create_timeline_header_grid_area,
  create_timeline_body_grid_area,
  create_side_panel_grid_area,
  create_timeline_col_container,
  create_timeline_hours_row_container,
  create_timeline_hour_outer,
  create_timeline_hour_inner,
};

pub fn spawn_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  // App container
  let all_father = cmd.spawn(create_app_grid_bundle()).id();
  // Col 1 row 1
  let empty_row = cmd.spawn(create_empty_grid_area()).id();
  // Col 1 row 2
  let top_bar = cmd.spawn(create_top_bar_grid_area())
    .with_children(|builder| {
        spawn_nested_text_bundle(builder, font.clone(), "Micronos", 18.)
      })
    .id();
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

  cmd.entity(all_father).push_children(&[empty_row,top_bar,timeline_header,timeline_body,side_modal]);
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

fn spawn_timeline_body_contents(cmd: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
  // Spawn a flex container to insert in the grid area, and hold the timeline elements
  let flex_col_container = cmd.spawn(create_timeline_col_container()).id();

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

  // Hour boxes
  let hours_container = cmd.spawn(create_timeline_hours_row_container()).id();
  for hour in Hour::VALUES {
    info!("time is {}", "potential".to_string());
    let hour_outer = cmd.spawn(create_timeline_hour_outer()).id();
    let hour_inner = cmd.spawn(create_timeline_hour_inner()).id();
     // TODO: load fonts once, centrally?
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    // TODO: implement with HourBox::new()
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