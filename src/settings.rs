
use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions};
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Window {
    pub fullscreen: bool,
    pub vsync: bool,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    pub fps: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Debug {
    pub show_fps: bool,
    pub show_fps_every: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub window: Window,
    pub system: System,
    pub debug: Debug,
}

impl Settings {
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                toml::from_str(contents.as_str())
                    .map_err(|e| e.to_string())
            },
            Err(message) => {
                Err(message.to_string())
            },
        }
    }

    pub fn default() -> Self {
        Self{
            window: Window{
                fullscreen: false,
                vsync: false,
                width: 1600,
                height: 900,
            },
            system: System{
                fps: 60,
            },
            debug: Debug{
                show_fps: true,
                show_fps_every: 3,
            }
        }
    }

    pub fn load_or_default(path: &std::path::Path) -> Self {
        match Self::load(path) {
            Ok(config) => config,
            Err(message) => {
                eprintln!("Error when loading config: {}", message);
                eprintln!("Using default config.");
                Self::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {

        let contents = toml::to_string(self)
            .map_err(|e| e.to_string())?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("Settings.toml")
            .map_err(|e| e.to_string())?;

        file.write_all(contents.as_bytes())
            .map_err(|e| e.to_string())?;


        Ok(())
    }
}
