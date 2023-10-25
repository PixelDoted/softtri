use softtri::{texture::TextureFormat, vertex::Vertex, SoftTriCanvas};

fn main() {
    let width = 960;
    let height = 720;

    let mut canvas = SoftTriCanvas::new(width, height, TextureFormat::RGB);
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

    let now = std::time::Instant::now();
    let mut ticks = 0;
    while now.elapsed().as_secs_f32() < 1.0 {
        canvas.draw_tri(&v0, &v1, &v2, None);
        ticks += 1;
    }
    println!("ticks {}, {}", ticks, now.elapsed().as_secs_f32());
}
