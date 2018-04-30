use std::collections::HashMap;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

pub struct TextureHandler<'a> {
    resource_path: &'static str,
    loaded_textures: HashMap<&'static str, Texture<'a>>,
    texture_creator: TextureCreator<WindowContext>
}

impl<'a: 'b, 'b > TextureHandler<'a> {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> TextureHandler<'a> {
        TextureHandler {
            resource_path: "Somewhere",
            loaded_textures: HashMap::new(),
            texture_creator: texture_creator
        }
    }

    pub fn load_texture(&'a mut self, texture_name: &'static str) -> &'b Texture {
        if !self.loaded_textures.contains_key(texture_name) {
            let loaded_texture = self.texture_creator
                .load_texture(texture_name)
                .expect("Failed to load texture");
            self.loaded_textures.insert(texture_name, loaded_texture);
        }

        self.loaded_textures.get(texture_name).clone().unwrap()
    }
}