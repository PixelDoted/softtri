use std::io::Write;

use softtri::{texture::TextureFormat, vertex::Vertex, SoftTriCanvas};

fn main() {
    let width = 1280;
    let height = 720;

    // Triangle
    let v0 = Vertex::new(
        [width as f32 / 2.0, 0.0].into(),
        [1.0, 0.0, 0.0, 1.0].into(),
        [0.0, 0.0].into(),
    );
    let v1 = Vertex::new(
        [0.0, height as f32].into(),
        [0.0, 1.0, 0.0, 1.0].into(),
        [1.0, 0.0].into(),
    );
    let v2 = Vertex::new(
        [width as f32, height as f32].into(),
        [0.0, 0.0, 1.0, 1.0].into(),
        [1.0, 1.0].into(),
    );

    // Canvas
    let mut canvas = SoftTriCanvas::new(width, height, TextureFormat::RGB);

    // Draw Triangle
    canvas.draw_tri(&v0, &v1, &v2, None);

    // Write texture to file
    let mut file = std::fs::File::create("examples/triangle.ppm").unwrap();
    file.write_all(format!("P6 {} {} 255\n", width, height).as_bytes())
        .unwrap();
    file.write_all(&canvas.buffer).unwrap();
    file.flush().unwrap();
}
