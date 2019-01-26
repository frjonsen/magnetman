use sdl2::{
  image::LoadTexture,
  render::{Texture, TextureCreator},
  video::WindowContext,
};
use std::collections::HashMap;

pub struct TextureHandler<'a> {
  resource_path: &'static str,
  loaded_textures: HashMap<&'static str, Texture<'a>>,
  texture_creator: TextureCreator<WindowContext>,
}

impl<'a: 'b, 'b> TextureHandler<'a> {
  pub fn new(texture_creator: TextureCreator<WindowContext>) -> TextureHandler<'a> {
    TextureHandler {
      resource_path: "Somewhere",
      loaded_textures: HashMap::new(),
      texture_creator,
    }
  }

  pub fn load_textures(&'a mut self, textures_names: &[&'static str]) {
    for texture in textures_names {
      let loaded_texture = self
        .texture_creator
        .load_texture(texture)
        .expect("Failed to load texture");
      self.loaded_textures.insert(texture, loaded_texture);
    }
  }

  pub fn get_texture(&'a self, texture_name: &'static str) -> &'b Texture {
    if !self.loaded_textures.contains_key(texture_name) {
      panic! {"Expected texture {} to be loaded, but no such texture found", texture_name};
    }

    self.loaded_textures.get(texture_name).clone().unwrap()
  }
}
