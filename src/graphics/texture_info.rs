pub struct TextureInfo {
  pub texture_name: &'static str,
  pub flip: bool,
  pub angle: i32,
}

impl TextureInfo {
  pub fn texture_name(&self) -> &'static str {
    self.texture_name.clone()
  }

  pub fn new(texture_name: &'static str, flip: bool, angle: i32) -> TextureInfo {
    TextureInfo {
      texture_name,
      flip,
      angle,
    }
  }
}
