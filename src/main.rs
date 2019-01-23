extern crate sdl2;

mod graphics;
mod physics;

pub fn main() {
    let mut engine = graphics::GraphicsEngine::new("test", 400, 200, 400, 200);
    engine.load_game_textures();
}
