use super::{texture_handler::TextureHandler, viewport::Viewport, Sprite};
use sdl2::{
  render::{Canvas, Texture},
  video::Window,
};

const GAME_TEXTURES: &'static [&'static str] = &["Hero_Run_1_B.png"];

pub struct GraphicsEngine<'a> {
  world_width: u32,
  world_height: u32,
  viewport: Viewport,
  canvas: Canvas<Window>,
  texture_handler: TextureHandler<'a>,
}

fn build_window(window_title: &str, window_width: u32, window_height: u32) -> Canvas<Window> {
  let video_subsystem = ::sdl2::init().unwrap().video().unwrap();
  let window = video_subsystem
    .window(window_title, window_width, window_height)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  window.into_canvas().build().unwrap()
}

impl<'a> GraphicsEngine<'a> {
  pub fn new(
    window_title: &str,
    window_width: u32,
    window_height: u32,
    world_width: u32,
    world_height: u32,
  ) -> GraphicsEngine {
    let window = build_window(window_title, window_width, window_height);
    let texture_creator = window.texture_creator();

    GraphicsEngine {
      world_width,
      world_height,
      viewport: Viewport { x: 0, y: 0 },
      canvas: window,
      texture_handler: TextureHandler::new(texture_creator),
    }
  }

  pub fn load_game_textures(&'a mut self) {
    self.texture_handler.load_textures(GAME_TEXTURES);
  }

  pub fn draw_screen(&mut self, sprites: &[&Sprite]) {
    self.canvas.clear();

    for s in sprites {
      let _t = self.texture_handler.get_texture(s.texture().texture_name);
    }
  }
}
