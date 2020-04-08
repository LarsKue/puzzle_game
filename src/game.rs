
use crate::{
    settings::{Settings},
    managers::{Managers},
};

pub struct Game {
    settings: Settings,
    managers: Managers,
}


impl Game {
    pub fn new() -> Self {
        let settings = Settings::load_or_default(std::path::Path::new("Settings.toml"));

        let managers = Managers::new(&settings);

        Self{settings, managers}
    }

    pub fn run(&mut self) -> Result<(), String> {

        self.managers.asset_manager.load_static_texture(&std::path::Path::new("example.png"));


        //Main loop
        // while let Some(event) = window.next() {
        //     window.draw_2d(&event, |mut context, window_graphics, _device| {
        //         clear(colors::WHITE, window_graphics);
        //         rectangle(colors::RED, [0.0, 0.0, 100.0, 100.0], context.transform, window_graphics);

        //         image(&texture, context.transform, window_graphics);
        //     });
        // }

        self.quit()?;

        Ok(())
    }

    fn quit(&mut self) -> Result<(), String> {
        // perform clean-up
        self.settings.save()?;

        Ok(())
    }
}