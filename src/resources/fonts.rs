
use bevy::prelude::*;

#[derive(Resource)]
pub struct Fonts {
  pub bold: Handle<Font>,
  pub medium: Handle<Font>,
}

impl FromWorld for Fonts {
  fn from_world(world: &mut World) -> Self {
    let bold = world.resource::<AssetServer>().load("fonts/FiraSans-Bold.ttf");
    let medium = world.resource::<AssetServer>().load("fonts/FiraMono-Medium.ttf");
    Self {
      bold,
      medium,
    }
  }
}