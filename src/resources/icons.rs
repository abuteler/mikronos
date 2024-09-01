
use bevy::prelude::*;

#[derive(Resource)]
pub struct Icons {
  pub collapse: Handle<Image>,
  pub expand: Handle<Image>,
}

impl FromWorld for Icons {
  fn from_world(world: &mut World) -> Self {
    let collapse = world.resource::<AssetServer>().load("icons/collapse_content_24dp_000000_FILL0_wght400_GRAD0_opsz24.png");
    let expand = world.resource::<AssetServer>().load("icons/expand_content_24dp_000000_FILL0_wght400_GRAD0_opsz24.png");
    Self {
      collapse,
      expand,
    }
  }
}