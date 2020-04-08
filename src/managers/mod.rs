
mod piston_manager;
mod asset_manager;

use piston_manager::PistonManager;
use asset_manager::AssetManager;

use crate::{
    settings::{Settings},
};

pub struct Managers {
    pub piston_manager: PistonManager,
    pub asset_manager: AssetManager,
}

impl Managers {
    pub fn new(settings: &Settings) -> Self {

        let mut piston_manager = PistonManager::new(settings);
        let asset_manager = AssetManager::new(piston_manager.mut_window());

        Self{piston_manager, asset_manager}
    }
}