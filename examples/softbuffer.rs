use std::num::NonZeroU32;

use softtri::{texture::TextureFormat, SoftTriCanvas, Vertex};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    // Winit
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_resizable(false);

    let PhysicalSize { width, height } = window.inner_size();

    // Triangle
    let (cw, ch) = (width as f32 * 0.5, height as f32 * 0.5);
    let v0 = Vertex::new(
        [cw, ch - 100.0].into(),
        [1.0, 0.0, 0.0, 1.0].into(),
        [0.0, 0.0].into(),
    );
    let v1 = Vertex::new(
        [cw - 100.0, ch + 100.0].into(),
        [0.0, 1.0, 0.0, 1.0].into(),
        [1.0, 0.0].into(),
    );
    let v2 = Vertex::new(
        [cw + 100.0, ch + 100.0].into(),
        [0.0, 0.0, 1.0, 1.0].into(),
        [1.0, 1.0].into(),
    );

    // Softbuffer
    // For this example we can initialize softbuffer here
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    surface
        .resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        )
        .unwrap();

    // Event Loop
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            let now = std::time::Instant::now();
            let mut canvas = SoftTriCanvas::new(width, height, TextureFormat::RGBA);
            canvas.draw_tri(&v0, &v1, &v2, None);

            let mut buffer = surface.buffer_mut().unwrap();
            for i in 0..(width * height) {
                let j = i * 4;
                let pixel = &canvas.buffer[j as usize..j as usize + 4];
                let red = pixel[0] as u32;
                let green = pixel[1] as u32;
                let blue = pixel[2] as u32;
                let alpha = pixel[3] as u32;

                buffer[i as usize] = blue | (green << 8) | (red << 16) | (alpha << 24);
            }

            buffer.present().unwrap();
            println!("present {}s", now.elapsed().as_secs_f32());
        }
        _ => (),
    });
}
