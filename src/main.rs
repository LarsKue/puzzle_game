
// extern crate piston;
extern crate piston_window;


mod colors;


use piston_window::*;

fn main() {
    let opengl  = OpenGL::V3_2;
    // let mut gl  = GlGraphics::new(opengl);

    let mut window : PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .expect("Failed to create window.");


    let texture_settings = TextureSettings::new();

    //Create the image object and attach a square Rectangle object inside.
    // let img   = image::Image::new().rect(square(0.0, 0.0, 200.0));
    //A texture to use with the image
    // let texture = Texture::from_path(Path::new("example.png"), &texture_settings).unwrap();

    let draw_state = DrawState::default();

    //Main loop
    while let Some(event) = window.next() {
        window.draw_2d(&event, |mut context, window_graphics, _device| {

            let texture = Texture::from_path(&mut context, std::path::Path::new("example.png"), Flip::None, &texture_settings).unwrap();
            clear(colors::WHITE, window_graphics);
            rectangle(colors::RED, [0.0, 0.0, 100.0, 100.0], context.transform, window_graphics);

            image(&texture, context.transform, window_graphics);
        });
    }
}