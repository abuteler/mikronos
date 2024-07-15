use bevy::prelude::*;

use crate::components::CurrentTimeText;

pub fn spawn_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  let app_container = NodeBundle {
    style: Style {
      display: Display::Grid,
      width: Val::Percent(100.),
      height: Val::Percent(100.),
      grid_template_columns: vec![
        GridTrack::px(208.),  // main app
        GridTrack::auto()     // hidden modal
      ],
      grid_template_rows: vec![
        GridTrack::auto(),      // app topbar: name + toolbar
        GridTrack::flex(1.0),   // timeline header
        GridTrack::px(170.),    // timeline body
      ],
      padding: UiRect::all(Val::Px(15.0)),
      ..default()
    },
    background_color: BackgroundColor(Color::BLACK),
    ..default()
  };
  // Spawn app container
  cmd.spawn(app_container)
    .with_children(|builder| {
      // Top bar
      builder
        .spawn(NodeBundle {
          style: Style {
            display: Display::Grid,
            grid_column: GridPlacement::span(1),
            ..default()
          },
          background_color: BackgroundColor(Color::WHITE),
          ..default()
        })
        .with_children(|builder| {
          spawn_nested_text_bundle(builder, font.clone(), "Micronos", 18.)
        });
      // Timeline header
      builder
        .spawn(NodeBundle {
          style: Style {
            display: Display::Grid,
            grid_column: GridPlacement::span(1),
            ..default()
          },
          background_color: BackgroundColor(Color::BLACK),
          ..default()
        })
        .with_children(|builder| {
          spawn_nested_text_bundle(builder, font.clone(), "Timeline header", 18.)
        });
      // Timeline body
      builder
        .spawn(NodeBundle {
          style: Style {
            display: Display::Grid,
            grid_column: GridPlacement::span(1),
            ..default()
          },
          background_color: BackgroundColor(Color::BLACK),
          ..default()
        })
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
          let sprite_handle = asset_server.load("SpectrumBg.png"); // 580 x 140 px
          // Timeline background image
          builder.spawn(SpriteBundle {
            texture: sprite_handle.clone(),
            ..default()
          });
        });
      // Side modal
      builder
        .spawn(NodeBundle {
          style: Style {
            display: Display::Grid,
            grid_column: GridPlacement::span(1),
            grid_row: GridPlacement::span(3),
            ..default()
          },
          background_color: BackgroundColor(Color::WHITE),
          ..default()
        })
        .with_children(|builder| {
          spawn_nested_text_bundle(builder, font.clone(), "Side modal", 18.)
        });
    });
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