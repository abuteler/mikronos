use bevy::prelude::*;

use crate::{resources::Icons, systems::toggle_window_decorations};

fn create_icon_button_bundle() -> ButtonBundle {
  ButtonBundle {
    style: Style {
        width: Val::Px(26.),
        height: Val::Px(26.),
        border: UiRect::all(Val::Px(1.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    border_color: BorderColor(Color::srgba_u8(90, 90, 90, 25)),
    background_color: BackgroundColor(Color::NONE),
    ..default()
  }
}

fn create_icon_image_bundle(texture: Handle<Image>) -> ImageBundle {
  ImageBundle {
    style: Style {
      width: Val::Px(24.),
      height: Val::Px(24.),
      ..default()
    },
    image: UiImage {
      texture,
      ..default()
    },
    ..default()
  }
}

pub fn spawn_icon_button(cmd: &mut Commands, icons: &Res<Icons>) {
  cmd.spawn((create_icon_button_bundle(), WindowDecorationsToggler)).with_children(|child_builder| {
    child_builder.spawn(create_icon_image_bundle(icons.collapse.clone()));
  });
  cmd.spawn(create_icon_button_bundle()).with_children(|child_builder| {
    child_builder.spawn(create_icon_image_bundle(icons.close.clone()));
  });
}

pub fn on_button_hover(
  mut interaction_query: Query<
    (&Interaction, &mut BorderColor),
    (Changed<Interaction>, With<WindowDecorationsToggler>),
  >,
  mut window: Query<&mut Window>,
  // _icons: &Res<Icons>
) {
  let (interaction, mut border_color) = interaction_query.single_mut();
  // let mut icon = image.texture
  match *interaction {
    Interaction::Pressed => {
      window.single_mut().decorations = !window.single_mut().decorations;
    }
    Interaction::Hovered => {
      border_color.0 = Color::srgba_u8(90, 90, 90, 25);
    }
    Interaction::None => {
      border_color.0 = Color::NONE;
    }
  }
}

pub fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut BorderColor),
    (Changed<Interaction>, With<WindowDecorationsToggler>),
  >,
  mut window: Query<&mut Window>,
  // _icons: &Res<Icons>
) {
  let (interaction, mut border_color) = interaction_query.single_mut();
  // let mut icon = image.texture
  match *interaction {
    Interaction::Pressed => {
      window.single_mut().decorations = !window.single_mut().decorations;
    }
    Interaction::Hovered => {
      border_color.0 = Color::srgba_u8(90, 90, 90, 25);
    }
    Interaction::None => {
      border_color.0 = Color::NONE;
    }
  }
}

pub fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut BorderColor),
    (Changed<Interaction>, With<WindowDecorationsToggler>),
  >,
  mut window: Query<&mut Window>,
  // _icons: &Res<Icons>
) {
  let (interaction, mut border_color) = interaction_query.single_mut();
  // let mut icon = image.texture
  match *interaction {
    Interaction::Pressed => {
      window.single_mut().decorations = !window.single_mut().decorations;
    }
    Interaction::Hovered => {
      border_color.0 = Color::srgba_u8(90, 90, 90, 25);
    }
    Interaction::None => {
      border_color.0 = Color::NONE;
    }
  }
}
