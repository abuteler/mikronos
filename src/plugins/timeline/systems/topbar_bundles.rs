use bevy::prelude::*;

pub fn create_flex_container_row() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Flex,
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::SpaceBetween,
      width: Val::Percent(100.),
      height: Val::Px(28.),
      padding: UiRect { top: Val::Px(2.), bottom: Val::Px(5.), ..default() },
      ..default()
    },
    background_color: BackgroundColor(Color::WHITE),
    border_radius: BorderRadius::px(0., 0., 0., 16.),
    ..default()
  }
}

pub fn create_flex_container_left() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Flex,
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::Baseline,
      justify_content: JustifyContent::SpaceBetween,
      width: Val::Percent(18.),
      height: Val::Px(28.),
      padding: UiRect { top: Val::Px(2.), bottom: Val::Px(2.), left: Val::Px(2.), ..default() },
      ..default()
    },
    border_radius: BorderRadius::px(0., 0., 0., 16.),
    ..default()
  }
}
pub fn create_flex_container_right() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Flex,
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::End,
      justify_content: JustifyContent::End,
      width: Val::Percent(82.),
      padding: UiRect { right: Val::Px(5.), ..default() },
      ..default()
    },
    border_radius: BorderRadius::px(0., 0., 0., 16.),
    ..default()
  }
}
