extern crate num;
extern crate sdl2;

use num::complex::Complex64;

static maxIterations: int = 50;
static threshold: f64 = 10.0;

enum SetPoint {
    Escaped(int),
    Contained
}

fn main() {
    sdl2::init(sdl2::InitVideo);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, sdl2::video::OpenGL) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    'main : loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'main,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'main
                    }
                },
                sdl2::event::NoEvent => break 'event,
                _ => {}
            }
        }
        let _ = renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
        let _ = renderer.clear();
        for x in range(0, 800) {
            for y in range(0, 600) {
                // TODO: Coordinate mapping function
                let p = Complex64 { re: -2.0 + (x as f64) / 200.0, im: -2.0 + (y as f64) / 150.0 };
                match escape(p) {
                    Escaped(c) => {
                        let l: u8 = (c * 5) as u8;
                        let _ = renderer.set_draw_color(sdl2::pixels::RGB(l, l, l));
                        let _ = renderer.draw_point(sdl2::rect::Point { x: x, y: y });
                    },
                    _ => {}
                }
            }
        }
        renderer.present();
    }

    sdl2::quit();
}

fn escape(c: Complex64) -> SetPoint {
    let mut z = Complex64 { re: 0.0, im: 0.0 };
    let mut iterations = 0;

    while z.norm() < threshold {
        z = z * z + c;
        iterations += 1;
        if iterations >= maxIterations {
            return Contained;
        }
    }

    Escaped(iterations)
}
