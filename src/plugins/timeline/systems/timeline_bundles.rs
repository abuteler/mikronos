use bevy::prelude::*;

pub const CONTAINER_PADDING_PX: f32 = 10.;
pub const TIMELINE_WIDTH_PX: f32 = 580.;
pub const HOURS_PADDING_PX: f32 = 7.;
pub const DIAL_WIDTH_PX: f32 = 2.;

pub fn create_flex_container_col() -> NodeBundle {
  NodeBundle {
    style: Style {
      position_type: PositionType::Relative,
      display: Display::Flex,
      flex_direction: FlexDirection::Column,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      width: Val::Px(TIMELINE_WIDTH_PX+CONTAINER_PADDING_PX*2.),
      height: Val::Percent(100.),
      padding: UiRect::all(Val::Px(CONTAINER_PADDING_PX)),
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
      height: Val::Px(28.),
      width: Val::Px(TIMELINE_WIDTH_PX),
      column_gap: Val::Px(HOURS_PADDING_PX),
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
