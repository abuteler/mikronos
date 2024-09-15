use bevy::prelude::*;

use crate::{resources::Icons, systems::toggle_window_decorations};
use super::topbar_bundles::{
  create_icon_button_bundle,
  create_icon_image_bundle,
};

#[derive(Component)]
pub struct WindowDecorationsToggler;

pub fn spawn_icon_buttons(cmd: &mut ChildBuilder, icons: &Res<Icons>) {
  cmd.spawn((create_icon_button_bundle(), WindowDecorationsToggler)).with_children(|child_builder| {
    child_builder.spawn(create_icon_image_bundle(icons.collapse.clone()));
  });
  cmd.spawn(create_icon_button_bundle()).with_children(|child_builder| {
    child_builder.spawn(create_icon_image_bundle(icons.close.clone()));
  });
}

// pub fn button_system(
//   mut interaction_query: Query<
//     (&Interaction, &mut BorderColor),
//     (Changed<Interaction>, With<WindowDecorationsToggler>),
//   >,
//   mut window: Query<&mut Window>,
//   // _icons: &Res<Icons>
// ) {
//   let (interaction, mut border_color) = interaction_query.single_mut();
//   // let mut icon = image.texture
//   match *interaction {
//     Interaction::Pressed => {
//       window.single_mut().decorations = !window.single_mut().decorations;
//     }
//     Interaction::Hovered => {
//       border_color.0 = Color::srgba_u8(90, 90, 90, 25);
//     }
//     Interaction::None => {
//       border_color.0 = Color::NONE;
//     }
//   }
// }
