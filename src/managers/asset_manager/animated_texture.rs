
type PistonTexture = piston_window::Texture<gfx_device_gl::Resources>;

/// Pixel Data Buffer for AnimatedTextures
#[derive(Clone)]
pub struct AnimatedTextureBuffer {
    buffer: PistonTexture,
    // meta data
    num_frames: u32,
    frames_per_frame: u32,
}

impl AnimatedTextureBuffer {
    pub fn new(buffer: PistonTexture, num_frames: u32, frames_per_frame: u32) -> Self {
        Self{buffer, num_frames, frames_per_frame}
    }
}

/// Buffer Referencing Type
#[derive(Copy, Clone)]
pub struct AnimatedTexture<'obj> {
    buffer: &'obj AnimatedTextureBuffer,
    current_frame: u32,
}

impl crate::traits::Draw for AnimatedTexture<'_> {
    fn draw(&self) {

    }
}

impl<'obj> AnimatedTexture<'obj> {
    pub fn new(buffer: &'obj AnimatedTextureBuffer) -> AnimatedTexture<'obj> {
        Self{buffer, current_frame: 0}
    }   
}