use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

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
    // Adjust divider width to be consistent
    let divider_width = 3;
    
    for pos in (start..end).step_by(step as usize) {
        let rect = if is_vertical {
            Rect::new(lane_offset - (divider_width / 2), pos, divider_width as u32, 20)
        } else {
            Rect::new(pos, lane_offset - (divider_width / 2), 20, divider_width as u32)
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
            get_intersection_boundaries().0,
            40,
        ),
        Direction::East => (
            6,
            (WINDOW_HEIGHT / 2) as i32 - (ROAD_HORIZONTAL_WIDTH / 2) as i32,
            false,
            get_intersection_boundaries().1,
            WINDOW_WIDTH as i32,
            40,
        ),
        Direction::North => (
            6,
            (WINDOW_WIDTH / 2) as i32 - (ROAD_VERTICAL_WIDTH / 2) as i32,
            true,
            0,
            get_intersection_boundaries().2,
            40,
        ),
        Direction::South => (
            6,
            (WINDOW_WIDTH / 2) as i32 - (ROAD_VERTICAL_WIDTH / 2) as i32,
            true,
            get_intersection_boundaries().3,
            WINDOW_HEIGHT as i32,
            40,
        ),
    };

    let lane_width = if is_vertical {
        ROAD_VERTICAL_WIDTH as i32 / lane_count
    } else {
        ROAD_HORIZONTAL_WIDTH as i32 / lane_count
    };

    // Draw outer boundaries of the road section
    let _outer_start = road_start;
    let _outer_end = road_start + (lane_width * lane_count);
    
    // Draw each lane divider (excluding outer boundaries)
    for lane in 1..lane_count {
        let lane_offset = road_start + (lane_width * lane);
        draw_lane_dividers(canvas, start, end, step, is_vertical, lane_offset);
    }

    // Create text renderer
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 24).unwrap();
    let texture_creator = canvas.texture_creator();

    // Draw lane numbers
    canvas.set_draw_color(Color::RGB(255, 255, 255));  // White color for text

    match direction {
        Direction::West => {
            // West section lane numbers (1-6 from top to bottom)
            for i in 0..lane_count {
                let x = (WINDOW_WIDTH as i32) / 2 - (ROAD_VERTICAL_WIDTH as i32) / 2 - 30;
                let y = (WINDOW_HEIGHT as i32) / 2 - (ROAD_HORIZONTAL_WIDTH as i32) / 2 + (lane_width as i32 * i);
                let surface = font.render(&format!("{}", i + 1)).blended(Color::WHITE).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                let rect = Rect::new(x, y, 20, 20);
                canvas.copy(&texture, None, Some(rect)).unwrap();
            }
        },
        Direction::East => {
            // East section lane numbers (1-6 from top to bottom)
            for i in 0..lane_count {
                let x = (WINDOW_WIDTH as i32) / 2 + (ROAD_HORIZONTAL_WIDTH as i32) / 2 + 10;
                let y = (WINDOW_HEIGHT as i32) / 2 - (ROAD_VERTICAL_WIDTH as i32) / 2 + (lane_width as i32 * i);
                let surface = font.render(&format!("{}", i + 1)).blended(Color::WHITE).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                let rect = Rect::new(x, y, 20, 20);
                canvas.copy(&texture, None, Some(rect)).unwrap();
            }
        },
        Direction::North => {
            // North section lane numbers (1-6 from left to right)
            for i in 0..lane_count {
                let x = (WINDOW_WIDTH as i32) / 2 - (ROAD_VERTICAL_WIDTH as i32) / 2 + (lane_width as i32 * i);
                let y = (WINDOW_HEIGHT as i32) / 2 - (ROAD_HORIZONTAL_WIDTH as i32) / 2 - 30;
                let surface = font.render(&format!("{}", i + 1)).blended(Color::WHITE).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                let rect = Rect::new(x, y, 20, 20);
                canvas.copy(&texture, None, Some(rect)).unwrap();
            }
        },
        Direction::South => {
            // South section lane numbers (1-6 from left to right)
            for i in 0..lane_count {
                let x = (WINDOW_WIDTH as i32) / 2 - (ROAD_VERTICAL_WIDTH as i32) / 2 + (lane_width as i32 * i);
                let y = (WINDOW_HEIGHT as i32) / 2 + (ROAD_HORIZONTAL_WIDTH as i32) / 2 + 10;
                let surface = font.render(&format!("{}", i + 1)).blended(Color::WHITE).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                let rect = Rect::new(x, y, 20, 20);
                canvas.copy(&texture, None, Some(rect)).unwrap();
            }
        },
    }
}

pub fn get_intersection_boundaries() -> (i32, i32, i32, i32) {
    (
        INTERSECTION_BOUNDARY_WEST as i32,
        INTERSECTION_BOUNDARY_EAST as i32,
        INTERSECTION_BOUNDARY_NORTH as i32,
        INTERSECTION_BOUNDARY_SOUTH as i32,
    )
}
