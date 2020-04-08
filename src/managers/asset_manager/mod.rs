
pub mod animated_texture;
pub mod static_texture;

use piston_window::{
    TextureSettings,
    TextureContext,
    PistonWindow,
    Flip,
};

type PistonTexture = piston_window::Texture<Resources>;

use gfx_device_gl::{
    Factory,
    Resources,
    CommandBuffer,
};

use std::{
    collections::{HashMap},
    path::{PathBuf, Path},
};

use static_texture::{
    StaticTextureBuffer,
    StaticTexture,
};

use animated_texture::{
    AnimatedTextureBuffer,
    AnimatedTexture,
};

/// Asset Manager
/// 
/// This class manages assets such as textures and animations.
/// The AssetManager holds hashed TextureBuffers, which contain pixel data
/// The user may request a static or animated Texture, which contains a reference
/// to the TextureBuffers. Thus, pixel data only needs to be allocated once
/// but can be used multiple times from anywhere within the program that has access
/// to the AssetManager.

pub struct AssetManager {
    settings: TextureSettings,
    context: TextureContext<Factory, Resources, CommandBuffer>,
    static_texture_buffers: HashMap<PathBuf, StaticTextureBuffer>,
    animated_texture_buffers: HashMap<PathBuf, AnimatedTextureBuffer>,
}

impl AssetManager {
    /// Create a new instance of the AssetManager
    pub fn new(window: &mut PistonWindow) -> Self {

        let settings = TextureSettings::new();

        let context = TextureContext{
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into()
        };

        let static_texture_buffers = HashMap::new();
        let animated_texture_buffers = HashMap::new();

        Self{settings, context, static_texture_buffers, animated_texture_buffers}
    }

    /// Internal syntactic sugar to load a PistonTexture
    fn load_piston_buffer(&mut self, path: &Path) -> PistonTexture {
        PistonTexture::from_path(&mut self.context, path, Flip::None, &self.settings).unwrap()
    }

    /// Load the pixel data of a StaticTexture
    pub fn load_static_texture(&mut self, path: &Path) {
        let piston_buffer = self.load_piston_buffer(path);
        let static_texture_buffer = StaticTextureBuffer::new(piston_buffer);
        self.static_texture_buffers.insert(path.to_path_buf(), static_texture_buffer);
    }

    /// Request a StaticTexture if its pixel data has already been loaded
    pub fn get_static_texture(&self, path: &Path) -> Option<StaticTexture> {
        self.static_texture_buffers.get(&path.to_path_buf()).map(StaticTexture::new)
    }

    /// Load and Request a StaticTexture
    pub fn load_and_get_static_texture(&mut self, path: &Path) -> StaticTexture {
        self.load_static_texture(path);
        self.get_static_texture(path).unwrap()
    }

    /// Check if a StaticTexture is already loaded and request or load it respectively
    pub fn get_or_load_static_texture(&mut self, path: &Path) -> StaticTexture {
        if self.is_static_texture_loaded(path) {
            self.get_static_texture(path).unwrap()
        } else {
            self.load_and_get_static_texture(path)
        }
    }

    pub fn is_static_texture_loaded(&self, path: &Path) -> bool {
        self.static_texture_buffers.contains_key(path)
    }

    /// Load the pixel data and meta data of an AnimatedTexture
    pub fn load_animated_texture(&mut self, path: &Path, num_frames: u32, frames_per_frame: u32) {
        let piston_buffer = self.load_piston_buffer(path);
        let animated_texture_buffer = AnimatedTextureBuffer::new(piston_buffer, num_frames, frames_per_frame);
        self.animated_texture_buffers.insert(path.to_path_buf(), animated_texture_buffer);
    }

    /// Request an AnimatedTexture if its pixel data and meta data has already been loaded
    pub fn get_animated_texture(&self, path: &Path) -> Option<AnimatedTexture> {
        self.animated_texture_buffers.get(&path.to_path_buf()).map(AnimatedTexture::new)
    }

    /// Load and Request an AnimatedTexture
    pub fn load_and_get_animated_texture(&mut self, path: &Path, num_frames: u32, frames_per_frame: u32) -> AnimatedTexture {
        self.load_animated_texture(path, num_frames, frames_per_frame);
        self.get_animated_texture(path).unwrap()
    }

    /// Check if an AnimatedTexture is already loaded and request or load it respectively
    pub fn get_or_load_animated_texture(&mut self, path: &Path, num_frames: u32, frames_per_frame: u32) -> AnimatedTexture {
        if self.is_animated_texture_loaded(path) {
            self.get_animated_texture(path).unwrap()
        } else {
            self.load_and_get_animated_texture(path, num_frames, frames_per_frame)
        }
    }

    pub fn is_animated_texture_loaded(&self, path: &Path) -> bool {
        self.animated_texture_buffers.contains_key(path)
    }

}