use crate::{Color, Point2D};

pub struct Texture {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
}

impl Texture {
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Self {
        Self {
            bytes: vec![0; width as usize * height as usize * format.bpp() as usize],
            width,
            height,
            format,
        }
    }

    pub(crate) fn get_mixed(&self, uv: Point2D) -> Color {
        let uv = uv * Point2D::new(self.width as f32 - 1.0, self.height as f32 - 1.0);
        // TODO: Linear filtering
        self.get_pixel(uv.x.round() as u32, uv.y.round() as u32)
            .into()
    }

    /// Gets a 0..1 rgba pixel
    pub(crate) fn get_pixel(&self, x: u32, y: u32) -> [f32; 4] {
        let i = (x + y * self.width) as usize;

        match self.format {
            TextureFormat::RGB => {
                let i = i * 3;
                [
                    self.bytes[i] as f32 / 255.0,
                    self.bytes[i + 1] as f32 / 255.0,
                    self.bytes[i + 2] as f32 / 255.0,
                    1.0,
                ]
            }
            TextureFormat::RGBA => {
                let i = i * 3;
                [
                    self.bytes[i] as f32 / 255.0,
                    self.bytes[i + 1] as f32 / 255.0,
                    self.bytes[i + 2] as f32 / 255.0,
                    self.bytes[i + 3] as f32 / 255.0,
                ]
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    RGB,
    RGBA,
}

impl TextureFormat {
    /// bytes per pixel
    pub fn bpp(&self) -> u32 {
        match self {
            TextureFormat::RGB => 3,
            TextureFormat::RGBA => 4,
        }
    }
}
