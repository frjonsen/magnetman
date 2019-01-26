use super::texture_info::TextureInfo;
use physics::velocity::Velocity;
use sdl2::rect::Rect;

pub struct SpriteInfo {
  texture_info: TextureInfo,
  velocity: Velocity,
  rectangle: Rect,
}

pub trait Sprite<'a> {
  fn texture(&self) -> &'a TextureInfo;
  fn velocity(&self) -> &'a Velocity;
  fn rectangle(&self) -> &'a Rect;
}
