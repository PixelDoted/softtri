//! ## Overview
//! a software triangle renderer
//!
//! ## Example
//! ```rust
//! let width = 1280;
//! let height = 720;
//!
//! let v0 = Vertex::new(
//!    [width as f32 / 2.0, 1.0].into(),
//!    [1.0, 0.0, 0.0, 1.0].into(),
//!    [0.0, 0.0].into(),
//! );
//! let v1 = Vertex::new(
//!    [1.0, height as f32 - 1.0].into(),
//!    [0.0, 1.0, 0.0, 1.0].into(),
//!    [1.0, 0.0].into(),
//! );
//! let v2 = Vertex::new(
//!    [width as f32 - 1.0, height as f32 - 1.0].into(),
//!    [0.0, 0.0, 1.0, 1.0].into(),
//!    [1.0, 1.0].into(),
//! );
//!
//! let mut canvas = SoftTriCanvas::new(width, height, TextureFormat::RGBA);
//! canvas.draw_tri(v0, v1, v2, None);
//!
//! // draw to a window/image using canvas.buffer
//! ```

mod color;
mod math;
mod point;
pub mod texture;
mod vertex;

pub use color::Color;
pub use point::Point2D;
use texture::{Texture, TextureFormat};
pub use vertex::Vertex;

/// A canvas to draw triangles onto
pub struct SoftTriCanvas {
    pub buffer: Vec<u8>,
    pub format: TextureFormat,
    pub size: [u32; 2],
}

impl SoftTriCanvas {
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Self {
        Self {
            buffer: vec![0; width as usize * height as usize * format.bpp() as usize],
            format,
            size: [width, height],
        }
    }

    /// Fills the background with `color`
    pub fn fill(&mut self, color: Color) {
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                let i = (x + y * self.size[0]) as usize;

                match self.format {
                    TextureFormat::RGB => {
                        let i = i * 3;
                        self.buffer[i] = (color.r * 255.0) as u8;
                        self.buffer[i + 1] = (color.g * 255.0) as u8;
                        self.buffer[i + 2] = (color.b * 255.0) as u8;
                    }
                    TextureFormat::RGBA => {
                        let i = i * 4;
                        self.buffer[i] = (color.r * 255.0) as u8;
                        self.buffer[i + 1] = (color.g * 255.0) as u8;
                        self.buffer[i + 2] = (color.b * 255.0) as u8;
                        self.buffer[i + 3] = (color.a * 255.0) as u8;
                    }
                }
            }
        }
    }

    /// Draws a triangle
    pub fn draw_tri(&mut self, v0: &Vertex, v1: &Vertex, v2: &Vertex, texture: Option<&Texture>) {
        // Normalize triangle
        let (min_x, max_x, min_y, max_y) = math::triangle_aabb(v0, v1, v2);

        // Draw triangle
        for y in min_y as i64..max_y as i64 {
            if y < 0 || y >= self.size[1] as i64 {
                continue;
            }

            for x in min_x as i64..=max_x as i64 {
                if x < 0 || x >= self.size[0] as i64 {
                    continue;
                }

                if let Some((u1, u2, det)) = math::barycentric(v0, v1, v2, x as f32, y as f32) {
                    let u3 = det - u1 - u2;
                    let mut rgba = v0.rgba * u1 / det + v1.rgba * u2 / det + v2.rgba * u3 / det;

                    if let Some(texture) = texture {
                        let uv = v0.uv * u1 / det + v1.uv * u2 / det + v2.uv * u3 / det;
                        // TODO: Linear Filtering
                        let pixel: Color = texture.get_pixel(uv.x as u32, uv.y as u32).into();
                        rgba *= pixel;
                    }

                    let pixel = self.get_pixel(x as u32, y as u32);
                    let out = math::blend_alpha(rgba, pixel);
                    self.set_pixel(x as u32, y as u32, out);
                }
            }
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> Color {
        let i = (x + y * self.size[0]) as usize;
        match self.format {
            TextureFormat::RGB => {
                let i = i * 3;
                Color::new(
                    self.buffer[i] as f32 / 255.0,
                    self.buffer[i + 1] as f32 / 255.0,
                    self.buffer[i + 2] as f32 / 255.0,
                    1.0,
                )
            }
            TextureFormat::RGBA => {
                let i = i * 4;
                Color::new(
                    self.buffer[i] as f32 / 255.0,
                    self.buffer[i + 1] as f32 / 255.0,
                    self.buffer[i + 2] as f32 / 255.0,
                    self.buffer[i + 3] as f32 / 255.0,
                )
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let i = (x + y * self.size[0]) as usize;
        match self.format {
            TextureFormat::RGB => {
                let i = i * 3;
                self.buffer[i] = (color.r * 255.0) as u8;
                self.buffer[i + 1] = (color.g * 255.0) as u8;
                self.buffer[i + 2] = (color.b * 255.0) as u8;
            }
            TextureFormat::RGBA => {
                let i = i * 4;
                self.buffer[i] = (color.r * 255.0) as u8;
                self.buffer[i + 1] = (color.g * 255.0) as u8;
                self.buffer[i + 2] = (color.b * 255.0) as u8;
                self.buffer[i + 3] = (color.a * 255.0) as u8;
            }
        }
    }
}
