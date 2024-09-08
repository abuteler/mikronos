use bevy::prelude::*;

pub fn create_flex_container_row() -> NodeBundle {
  NodeBundle {
    style: Style {
      display: Display::Flex,
      flex_direction: FlexDirection::Row,
      align_items: AlignItems::Start,
      justify_content: JustifyContent::SpaceBetween,
      width: Val::Percent(100.),
      height: Val::Px(75.),
      padding: UiRect::all(Val::Px(5.)),
      ..default()
    },
    // background_color: BackgroundColor(Color::srgb_u8(25, 32, 42)),
    // border_radius: BorderRadius::px(0., 0., 0., 16.),
    ..default()
  }
}
