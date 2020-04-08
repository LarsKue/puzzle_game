
extern crate piston_window;
extern crate gfx_device_gl;

mod game;
mod managers;
mod math;
mod traits;
mod settings;
mod colors;

fn main() -> Result<(), String> {

    let mut game = game::Game::new();

    game.run()?;

    Ok(())
}