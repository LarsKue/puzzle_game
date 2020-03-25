
extern crate piston_window;

use piston_window::*;

fn main() {

    let white = [1.0; 4];
    // let black = [0.0, 0.0, 0.0, 1.0];
    let red = [1.0, 0.0, 0.0, 1.0];

    let mut window : PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .expect("Failed to create window.");

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear(white, graphics);
            rectangle(red, [0.0, 0.0, 100.0, 100.0], context.transform, graphics);
        });
    }
}
