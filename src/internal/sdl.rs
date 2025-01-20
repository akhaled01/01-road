use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl};

use super::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn init_sdl() -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("01-road", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    (sdl_context, canvas)
}
