
type PistonTexture = piston_window::Texture<gfx_device_gl::Resources>;

/// Pixel Data Buffer for StaticTextures
#[derive(Clone)]
pub struct StaticTextureBuffer {
    buffer: PistonTexture,
}

impl StaticTextureBuffer {
    pub fn new(buffer: PistonTexture) -> Self {
        Self{buffer}
    }
}

/// Buffer Referencing Type
#[derive(Copy, Clone)]
pub struct StaticTexture<'obj> {
    buffer: &'obj StaticTextureBuffer,
}

impl crate::traits::Draw for StaticTexture<'_> {
    fn draw(&self) {

    }
}

impl<'obj> StaticTexture<'obj> {
    pub fn new(buffer: &'obj StaticTextureBuffer) -> StaticTexture<'obj> {
        Self{buffer}
    }
}