extern crate sdl2;

mod graphics;
mod physics;

pub fn main() {
    let engine = graphics::GraphicsEngine::new("test", 0,0,0,0);
}