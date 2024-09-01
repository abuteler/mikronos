use bevy::prelude::*;

pub fn create_row_container() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Flex,
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::SpaceEvenly,
      width: Val::Percent(100.),
      height: Val::Px(28.),
      padding: UiRect { top: Val::Px(2.), bottom: Val::Px(0.), ..default() },
      ..default()
    },
    background_color: BackgroundColor(Color::WHITE),
    border_radius: BorderRadius::px(0., 0., 0., 16.),
    ..default()
  }
}

pub fn create_title(text: &str, font: Handle<Font>) -> TextBundle {
  TextBundle::from_section(
    text,
    TextStyle {
      font,
      font_size: 18.,
      color: Color::BLACK,
    },
  )
}
