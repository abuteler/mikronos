use bevy::prelude::*;

#[derive(Resource)]
pub struct TimelineAssets {
  pub spectrum_bg: Handle<Image>,
}

impl FromWorld for TimelineAssets {
  fn from_world(world: &mut World) -> Self {
    let spectrum_bg = world.resource::<AssetServer>().load("SpectrumBg.png");
    Self {
      spectrum_bg,
    }
  }
}
