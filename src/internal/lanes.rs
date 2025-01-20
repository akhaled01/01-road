use sdl2::{rect::Rect, render::Canvas, video::Window};

use super::constants::*;

/// Enum to represent directions for lane drawing
#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Helper to draw lane dividers for a specific direction
fn draw_lane_dividers(
    canvas: &mut Canvas<Window>,
    start: i32,
    end: i32,
    step: i32,
    is_vertical: bool,
    lane_offset: i32,
) {
    for pos in (start..end).step_by(step as usize) {
        let rect = if is_vertical {
            Rect::new(lane_offset, pos, 5, 20)
        } else {
            Rect::new(pos, lane_offset, 20, 5)
        };

        canvas.fill_rect(rect).unwrap_or_else(|e| {
            eprintln!(
                "Error drawing lane divider at pos={} offset={}: {}",
                pos, lane_offset, e
            )
        });
    }
}

/// Draw lanes for a specific direction
pub fn draw_lanes(canvas: &mut Canvas<Window>, direction: Direction) {
    let (lane_count, road_start, is_vertical, start, end, step) = match direction {
        Direction::West => (
            6,
            (WINDOW_HEIGHT / 2) as i32 - (ROAD_HORIZONTAL_WIDTH / 2) as i32,
            false,
            0,
            INTERSECTION_BOUNDARY_WEST as i32,
            40,
        ),
        Direction::East => (
            6,
            (WINDOW_HEIGHT / 2) as i32 - (ROAD_HORIZONTAL_WIDTH / 2) as i32,
            false,
            INTERSECTION_BOUNDARY_EAST as i32,
            WINDOW_WIDTH as i32,
            40,
        ),
        Direction::North => (
            6,
            (WINDOW_WIDTH / 2) as i32 - (ROAD_VERTICAL_WIDTH / 2) as i32,
            true,
            0,
            INTERSECTION_BOUNDARY_NOTRTH as i32,
            40,
        ),
        Direction::South => (
            6,
            (WINDOW_WIDTH / 2) as i32 - (ROAD_VERTICAL_WIDTH / 2) as i32,
            true,
            INTERSECTION_BOUNDARY_SOUTH as i32,
            WINDOW_HEIGHT as i32,
            40,
        ),
    };

    let lane_width = if is_vertical {
        ROAD_VERTICAL_WIDTH as i32 / lane_count
    } else {
        ROAD_HORIZONTAL_WIDTH as i32 / lane_count
    };

    for lane in 0..lane_count {
        let lane_offset = road_start + (lane_width * lane) + (lane_width / 2) - 5;
        draw_lane_dividers(canvas, start, end, step, is_vertical, lane_offset);
    }
}
