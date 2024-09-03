
use bevy::prelude::*;

#[derive(Resource)]
pub struct Icons {
  pub close: Handle<Image>,
  pub collapse: Handle<Image>,
  pub expand: Handle<Image>,
  pub fort: Handle<Image>,
  pub shield: Handle<Image>,
}

impl FromWorld for Icons {
  fn from_world(world: &mut World) -> Self {
    let close = world.resource::<AssetServer>().load("icons/close_24dp_000000_FILL0_wght400_GRAD0_opsz24.png");
    let collapse = world.resource::<AssetServer>().load("icons/collapse_content_24dp_000000_FILL0_wght400_GRAD0_opsz24.png");
    let expand = world.resource::<AssetServer>().load("icons/expand_content_24dp_000000_FILL0_wght400_GRAD0_opsz24.png");
    let fort = world.resource::<AssetServer>().load("icons/fort_24dp_000000_FILL1_wght400_GRAD0_opsz24.png");
    let shield = world.resource::<AssetServer>().load("icons/arming_countdown_24dp_000000_FILL1_wght400_GRAD0_opsz24.png");
    Self {
      close,
      collapse,
      expand,
      fort,
      shield,
    }
  }
}