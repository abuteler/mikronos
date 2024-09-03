use bevy::prelude::*;

pub fn create_text_bundle(text: &str, font: Handle<Font>, font_size: f32, color: Color) -> TextBundle {
  TextBundle::from_section(
    text,
    TextStyle {
      font,
      font_size,
      color,
    },
  )
}
