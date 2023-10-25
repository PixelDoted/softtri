use crate::{color::Color, point::Point2D};

pub struct Vertex {
    pub point: Point2D,
    pub rgba: Color,
    pub uv: Point2D,
}

impl Vertex {
    pub fn new(point: Point2D, rgba: Color, uv: Point2D) -> Self {
        Self { point, rgba, uv }
    }
}
