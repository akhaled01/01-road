use internal::{
    environment::draw_env,
    sdl::init_sdl,
    vehicles::Vehicle,
};
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod internal;

fn main() {
    let (sdl_ctx, mut canvas) = init_sdl();
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    let mut vehicles = Vec::new();
    let mut frame_count = 0;

    'running: loop {
        // Handle events
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

        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw environment
        draw_env(&mut canvas);

        // Spawn new vehicles (randomly)
        frame_count += 1;
        if frame_count % 60 == 0 && vehicles.len() < 20 {  // Spawn every ~1 second if less than 20 vehicles
            if rng.gen_bool(0.3) {  // 30% chance to spawn
                vehicles.push(Vehicle::random());
            }
        }

        // Update and render vehicles
        vehicles.retain_mut(|vehicle| {
            vehicle.move_vehicle();
            vehicle.render(&mut canvas);
            
            // Remove vehicles that are off-screen
            let x = vehicle.pos_x;
            let y = vehicle.pos_y;
            x >= -50
                && x <= canvas.window().size().0 as i32 + 50
                && y >= -50
                && y <= canvas.window().size().1 as i32 + 50
        });

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
