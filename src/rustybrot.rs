extern crate sdl2;

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
        let _ = renderer.set_draw_color(sdl2::pixels::RGB(200, 200, 100));
        let point = sdl2::rect::Point { x: 50, y: 100 };
        let _ = renderer.draw_point(point);
        renderer.present();
    }

    sdl2::quit();
}
