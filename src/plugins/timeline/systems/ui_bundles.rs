use bevy::prelude::*;

pub fn create_app_grid_bundle() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Grid,
      width: Val::Percent(100.),
      height: Val::Percent(100.),
      grid_template_columns: vec![
        GridTrack::px(610.),  // main app
        GridTrack::auto()     // Side modal at flex width
      ],
      grid_template_rows: vec![
        GridTrack::flex(1.),  // to be transparent so Side modal can be taller
        GridTrack::px(25.),   // app topbar: name + toolbar
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

pub fn create_top_bar_grid_area() -> NodeBundle {
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
    background_color: BackgroundColor(Color::WHITE),
    ..default()
  }
}

pub fn create_timeline_col_container() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Flex,
      flex_direction: FlexDirection::Column,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      column_gap: Val::Px(3.),
      width: Val::Percent(100.),
      height: Val::Percent(100.),
      padding: UiRect::all(Val::Px(5.0)).with_bottom(Val::Px(10.)),
      ..default()
    },
    background_color: BackgroundColor(Color::BLACK),
    border_radius: BorderRadius::px(0., 0., 0., 16.),
    ..default()
  }
}

pub fn create_timeline_hours_row_container() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Flex,
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::SpaceEvenly,
      width: Val::Percent(100.),
      height: Val::Px(28.),
      ..default()
    },
    background_color: BackgroundColor(Color::BLACK),
    border_radius: BorderRadius::px(0., 0., 0., 16.),
    ..default()
  }
}

pub fn create_timeline_hour_outer() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Block,
      width: Val::Percent(3.),
      height: Val::Px(24.),
      padding: UiRect { top: Val::Px(2.), bottom: Val::Px(0.), ..default() },
      ..default()
    },
    background_color: BackgroundColor(Color::WHITE),
    ..default()
  }
}

pub fn create_timeline_hour_inner() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Flex,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      width: Val::Percent(100.),
      height: Val::Px(22.),
      padding: UiRect { top: Val::Px(4.), ..default() },
      ..default()
    },
    background_color: BackgroundColor(Color::BLACK),
    ..default()
  }
}
