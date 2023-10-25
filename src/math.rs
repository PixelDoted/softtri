use crate::{color::Color, vertex::Vertex};

pub fn blend_alpha(a: Color, b: Color) -> Color {
    if a.a == 0.0 {
        return b;
    } else if b.a == 0.0 || a.a == 1.0 {
        return a;
    }

    let inv_aalpha = 1.0 - a.a;
    let alpha = a.a + b.a * inv_aalpha;
    Color::new(
        (a.r * a.a + b.r * b.b * inv_aalpha) / alpha,
        (a.g * a.a + b.g * b.b * inv_aalpha) / alpha,
        (a.b * a.a + b.b * b.b * inv_aalpha) / alpha,
        alpha,
    )
}

pub fn triangle_aabb(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> (f32, f32, f32, f32) {
    let (min_x, max_x) = (
        v0.point.x.min(v1.point.x).min(v2.point.x),
        v0.point.x.max(v1.point.x).max(v2.point.x),
    );
    let (min_y, max_y) = (
        v0.point.y.min(v1.point.y).min(v2.point.y),
        v0.point.y.max(v1.point.y).max(v2.point.y),
    );

    (min_x, max_x, min_y, max_y)
}

pub fn barycentric(
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    x: f32,
    y: f32,
) -> Option<(f32, f32, f32)> {
    let (x1, y1) = (v0.point.x, v0.point.y);
    let (x2, y2) = (v1.point.x, v1.point.y);
    let (x3, y3) = (v2.point.x, v2.point.y);

    let det = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
    let u1 = (y2 - y3) * (x - x3) + (x3 - x2) * (y - y3);
    let u2 = (y3 - y1) * (x - x3) + (x1 - x3) * (y - y3);

    let (u11, u22) = (u1 / det, u2 / det);
    let u33 = 1.0 - u11 - u22;
    if 0.0 < u11 && u11 < 1.0 && 0.0 < u22 && u22 < 1.0 && 0.0 < u33 && u33 < 1.0 {
        Some((u1, u2, det))
    } else {
        None
    }
}
