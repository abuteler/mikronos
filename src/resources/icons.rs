
use bevy::prelude::*;

#[derive(Resource)]
pub struct Icons {
  pub app_icon: Handle<Image>,
  pub close: Handle<Image>,
  pub collapse: Handle<Image>,
  pub expand: Handle<Image>,
  pub fort: Handle<Image>,
}

impl FromWorld for Icons {
  fn from_world(world: &mut World) -> Self {
    let app_icon = world.resource::<AssetServer>().load("icons/app_icon_48px.png");
    let close = world.resource::<AssetServer>().load("icons/close_24dp_000000_FILL0_wght400_GRAD0_opsz24.png");
    let collapse = world.resource::<AssetServer>().load("icons/collapse_content_24dp_000000_FILL0_wght400_GRAD0_opsz24.png");
    let expand = world.resource::<AssetServer>().load("icons/expand_content_24dp_000000_FILL0_wght400_GRAD0_opsz24.png");
    let fort = world.resource::<AssetServer>().load("icons/fort_24dp_000000_FILL1_wght400_GRAD0_opsz24.png");
    Self {
      app_icon,
      close,
      collapse,
      expand,
      fort,
    }
  }
}
