use internal::environment::draw_env;
use internal::sdl::init_sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod internal;

fn main() {
    let (sdl_ctx, mut canvas) = init_sdl();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    draw_env(&mut canvas);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
