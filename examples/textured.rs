use std::io::Write;

use softtri::{
    texture::{Texture, TextureFormat},
    SoftTriCanvas, Vertex,
};

fn main() {
    let width = 250;
    let height = 250;

    // Triangle
    let cx = width as f32 * 0.5;
    let cy = height as f32 * 0.5;
    let v0 = Vertex::new(
        [cx - 100.0, cy - 100.0].into(),
        [1.0, 1.0, 1.0, 1.0].into(),
        [0.0, 0.0].into(),
    );
    let v1 = Vertex::new(
        [cx - 100.0, cy + 100.0].into(),
        [0.0, 1.0, 0.0, 1.0].into(),
        [1.0, 0.0].into(),
    );
    let v2 = Vertex::new(
        [cx + 100.0, cy + 100.0].into(),
        [0.0, 0.0, 1.0, 1.0].into(),
        [1.0, 1.0].into(),
    );
    let v3 = Vertex::new(
        [cx + 100.0, cy - 100.0].into(),
        [1.0, 0.0, 0.0, 1.0].into(),
        [1.0, 0.0].into(),
    );

    // Texture
    let mut texture = Texture::new(16, 16, TextureFormat::RGB);
    for pixel in texture.bytes.chunks_mut(3) {
        pixel[0] = rand::random();
        pixel[1] = rand::random();
        pixel[2] = rand::random();
    }

    // Canvas
    let mut canvas = SoftTriCanvas::new(width, height, TextureFormat::RGB);

    // Draw Triangle
    canvas.draw_tri(&v0, &v1, &v2, Some(&texture));
    canvas.draw_tri(&v0, &v3, &v2, Some(&texture));

    // Write texture to file
    let mut file = std::fs::File::create("examples/textured.ppm").unwrap();
    file.write_all(format!("P6 {} {} 255\n", width, height).as_bytes())
        .unwrap();
    file.write_all(&canvas.buffer).unwrap();
    file.flush().unwrap();
}
