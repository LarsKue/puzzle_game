
use piston_window::*;

use crate::{
    settings::{Settings},
};

pub struct PistonManager {
    open_gl: OpenGL,
    window: PistonWindow,
}

impl PistonManager {
    pub fn new(settings: &Settings) -> Self {
        let open_gl  = OpenGL::V3_2;

        let window : PistonWindow = WindowSettings::new("Puzzle Game", [settings.window.width, settings.window.height])
        .fullscreen(settings.window.fullscreen)
        .vsync(settings.window.vsync)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .expect("Failed to create window.");

        Self{open_gl, window}
    }

    pub fn window(&self) -> &PistonWindow {
        &self.window
    }

    pub fn mut_window(&mut self) -> &mut PistonWindow {
        &mut self.window
    }
}