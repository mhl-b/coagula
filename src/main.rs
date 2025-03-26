mod geometry;

use geometry::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::FPoint;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn main() {
    // Constants for better readability
    const WINDOW_TITLE: &str = "rust-sdl2 demo";
    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 600;
    const FRAME_DURATION: std::time::Duration = std::time::Duration::new(0, 1_000_000_000u32 / 120);

    const SAGE: Color = Color::RGB(197, 195, 146);
    const HONEYDEW: Color = Color::RGB(235, 245, 223);
    const BLACK: Color = Color::RGB(0, 0, 0);

    // Initialize SDL2 and create window and canvas
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // Layout setup
    let origin = FPoint::new(16., 16.);
    let mut size = FPoint::new(16., 16.);
    let mut layout = Layout::new(ORIENTATION_POINTY, origin, size);

    // Initial rendering
    canvas.set_draw_color(SAGE);
    canvas.clear();
    canvas.present();

    let mut mouse_pos = FPoint::new(0., 0.);
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Main loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    size = match code {
                        Keycode::W => FPoint::new(size.x(), size.y() + 1.),
                        Keycode::S => FPoint::new(size.x(), size.y() - 1.),
                        Keycode::A => FPoint::new(size.x() - 1., size.y()),
                        Keycode::D => FPoint::new(size.x() + 1., size.y()),
                        Keycode::Escape => break 'running,
                        _ => size,
                    };
                    layout = Layout::new(ORIENTATION_POINTY, size, origin); // Update layout
                }
                Event::Quit { .. } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = FPoint::new(x as f32, y as f32);
                }
                _ => {}
            }
        }

        // Clear and redraw the canvas
        canvas.set_draw_color(SAGE);
        canvas.clear();
        draw_layout(&mut canvas, &layout, BLACK, HONEYDEW, mouse_pos);
        canvas.present();

        // Frame rate control
        ::std::thread::sleep(FRAME_DURATION);
    }
}

// Helper function to encapsulate drawing the layout
fn draw_layout(
    canvas: &mut Canvas<Window>,
    layout: &Layout,
    grid_color: Color,
    hover_color: Color,
    mouse_pos: FPoint,
) {
    let hovered_hex = layout.point_to_hex(mouse_pos);

    // Draw layout grid
    let grid_rect = Rect::new(IHEX_0, 20, 20);
    canvas.set_draw_color(grid_color);
    layout
        .hex_centers(grid_rect.hex_iter())
        .map(|center| layout.grid_corners(center))
        .for_each(|corners| {
            canvas.draw_flines(corners.as_slice()).unwrap();
            canvas.draw_fline(corners[0], corners[5]).unwrap();
        });

    // Highlight hovered hex
    let point = Point(hovered_hex);
    canvas.set_draw_color(hover_color);
    layout
        .hex_centers(point.hex_iter())
        .map(|center| layout.grid_corners(center))
        .for_each(|corners| {
            canvas.draw_flines(corners.as_slice()).unwrap();
            canvas.draw_fline(corners[0], corners[5]).unwrap();
        });
}
