use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use super::{
    constants::*,
    lanes::{draw_lanes, Direction},
};

pub fn draw_env(canvas: &mut Canvas<Window>) {
    // Set the background color to dark gray
    canvas.set_draw_color(Color::RGB(50, 50, 50));

    // Draw vertical road
    let vertical_road_pos = (WINDOW_WIDTH / 2) as i32 - (ROAD_VERTICAL_WIDTH / 2) as i32;
    canvas
        .fill_rect(Rect::new(
            vertical_road_pos,
            0,
            ROAD_VERTICAL_WIDTH,
            WINDOW_HEIGHT,
        ))
        .unwrap_or_else(|e| eprintln!("Error drawing vertical road: {}", e));

    // Draw horizontal road
    let horizontal_road_pos = (WINDOW_HEIGHT / 2) as i32 - (ROAD_HORIZONTAL_WIDTH / 2) as i32;
    canvas
        .fill_rect(Rect::new(
            0,
            horizontal_road_pos,
            WINDOW_WIDTH,
            ROAD_HORIZONTAL_WIDTH,
        ))
        .unwrap_or_else(|e| eprintln!("Error drawing horizontal road: {}", e));

    // Set lane divider color to yellow
    canvas.set_draw_color(Color::RGB(255, 255, 50));

    // Draw lanes for each direction
    draw_lanes(canvas, Direction::West);
    draw_lanes(canvas, Direction::North);
    draw_lanes(canvas, Direction::South);
    draw_lanes(canvas, Direction::East);
}
