use bevy::prelude::*;

pub fn create_app_grid_bundle() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Grid,
      width: Val::Percent(100.),
      height: Val::Percent(100.),
      grid_template_columns: vec![
        GridTrack::px(620.),  // main app
        GridTrack::auto()     // Side modal at flex width
      ],
      grid_template_rows: vec![
        GridTrack::flex(1.),  // to be transparent so Side modal can be taller
        GridTrack::px(28.),   // app topbar: name + toolbar
        GridTrack::px(75.),   // timeline header
        GridTrack::px(180.),  // timeline body
      ],
      padding: UiRect::all(Val::Px(15.0)),
      ..default()
    },
    background_color: BackgroundColor::DEFAULT,
    border_radius: BorderRadius::px(0., 16., 0., 16.),
    ..default()
  }
}

pub fn create_empty_grid_area() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Grid,
      grid_column: GridPlacement::start_end(1, 2),
      grid_row: GridPlacement::start_end(1, 2),
      ..default()
    },
    background_color: BackgroundColor::DEFAULT,
    ..default()
  }
}

pub fn create_topbar_grid_area() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Grid,
      grid_column: GridPlacement::start_end(1, 2),
      grid_row: GridPlacement::start_end(2, 3),
      ..default()
    },
    background_color: BackgroundColor(Color::WHITE),
    ..default()
  }
}

pub fn create_timeline_header_grid_area() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Grid,
      grid_column: GridPlacement::start_end(1, 2),
      grid_row: GridPlacement::start_end(3, 4),
      ..default()
    },
    background_color: BackgroundColor(Color::srgb_u8(25, 32, 42)),
    ..default()
  }
}

pub fn create_timeline_body_grid_area() -> NodeBundle {
  NodeBundle{
    style: Style {
      display: Display::Grid,
      grid_column: GridPlacement::start_end(1, 2),
      grid_row: GridPlacement::start_end(4, 5),
      ..default()
    },
    border_radius: BorderRadius::px(0., 0., 0., 16.),
    background_color: BackgroundColor::DEFAULT,
    ..default()
  }
}

pub fn create_side_panel_grid_area() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Grid,
      grid_column: GridPlacement::start_end(2, 3),
      grid_row: GridPlacement::start_end(1, 5),
      ..default()
    },
    background_color: BackgroundColor(Color::srgba_u8(180, 180, 210, 75)),
    ..default()
  }
}

