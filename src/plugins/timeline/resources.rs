use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TimelineAssets {
  pub background_handle: Option<Handle<Image>>,
}
