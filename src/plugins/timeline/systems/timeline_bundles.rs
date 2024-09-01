use bevy::prelude::*;

pub fn create_col_container() -> NodeBundle {
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

pub fn create_hours_row_container() -> NodeBundle {
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

pub fn create_hour_outer() -> NodeBundle {
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

pub fn create_hour_inner() -> NodeBundle {
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
