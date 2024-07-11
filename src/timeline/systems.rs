use bevy::prelude::*;

pub fn load_bg(mut cmd: Commands, asset_server: Res<AssetServer>) {
  let sprite_handle = asset_server.load("SpectrumBg.png");

  cmd.spawn(SpriteBundle {
    texture: sprite_handle.clone(),
    ..default()
  });
}
