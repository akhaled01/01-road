use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use super::lanes::Direction;
use super::constants::*;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum TurnDirection {
    Left,
    Straight,
    Right,
}

pub struct Vehicle {
    pub pos_x: i32,
    pub pos_y: i32,
    pub direction: Direction,
    pub lane: i32,
    pub turn_direction: TurnDirection,
    pub turning: bool,
}

impl Vehicle {
    pub fn new(direction: Direction, lane: i32, turn_direction: TurnDirection) -> Self {
        let (pos_x, pos_y) = Self::get_start_position(direction, lane);
        
        Vehicle {
            pos_x,
            pos_y,
            direction,
            lane,
            turn_direction,
            turning: false,
        }
    }

    fn get_start_position(direction: Direction, lane: i32) -> (i32, i32) {
        // Convert lane index (0-5) to actual position
        let lane_offset = lane * LANE_WIDTH + (LANE_WIDTH / 2);
        
        match direction {
            Direction::North => {
                // For North, lanes go from left to right (1-6)
                let x = (WINDOW_WIDTH as i32 / 2) - (ROAD_VERTICAL_WIDTH as i32 / 2) + lane_offset;
                let y = WINDOW_HEIGHT as i32;
                (x, y)
            },
            Direction::South => {
                // For South, lanes go from left to right (1-6)
                let x = (WINDOW_WIDTH as i32 / 2) - (ROAD_VERTICAL_WIDTH as i32 / 2) + lane_offset;
                let y = 0;
                (x, y)
            },
            Direction::East => {
                // For East, lanes go from top to bottom (1-6)
                let x = 0;
                let y = (WINDOW_HEIGHT as i32 / 2) - (ROAD_VERTICAL_WIDTH as i32 / 2) + lane_offset;
                (x, y)
            },
            Direction::West => {
                // For West, lanes go from top to bottom (1-6)
                let x = WINDOW_WIDTH as i32;
                let y = (WINDOW_HEIGHT as i32 / 2) - (ROAD_VERTICAL_WIDTH as i32 / 2) + lane_offset;
                (x, y)
            }
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
        let direction = directions[rng.gen_range(0..4)];
        
        // Get valid lanes and their turn directions based on direction
        let (lane, turn_direction) = match direction {
            Direction::East => {
                // East: Only Lanes 4,5,6 (inbound)
                let lane_idx = rng.gen_range(0..LANES_PER_SIDE);
                let lane = EAST_INBOUND_START + lane_idx;
                let turn_direction = match lane_idx {
                    0 => TurnDirection::Left,     // Lane 4 -> North 4
                    1 => TurnDirection::Straight, // Lane 5 -> East 5
                    2 => TurnDirection::Right,    // Lane 6 -> South 1
                    _ => unreachable!()
                };
                (lane, turn_direction)
            },
            Direction::South => {
                // South: Only Lanes 1,2,3 (inbound)
                let lane_idx = rng.gen_range(0..LANES_PER_SIDE);
                let lane = SOUTH_INBOUND_START + lane_idx;
                let turn_direction = match lane_idx {
                    0 => TurnDirection::Right,    // Lane 1 -> West 1
                    1 => TurnDirection::Straight, // Lane 2 -> South 2
                    2 => TurnDirection::Left,     // Lane 3 -> East 4
                    _ => unreachable!()
                };
                (lane, turn_direction)
            },
            Direction::West => {
                // West: Only Lanes 1,2,3 (inbound)
                let lane_idx = rng.gen_range(0..LANES_PER_SIDE);
                let lane = WEST_INBOUND_START + lane_idx;
                let turn_direction = match lane_idx {
                    0 => TurnDirection::Right,    // Lane 1 -> North 6
                    1 => TurnDirection::Straight, // Lane 2 -> West 2
                    2 => TurnDirection::Left,     // Lane 3 -> South 3
                    _ => unreachable!()
                };
                (lane, turn_direction)
            },
            Direction::North => {
                // North: Only Lanes 4,5,6 (inbound)
                let lane_idx = rng.gen_range(0..LANES_PER_SIDE);
                let lane = NORTH_INBOUND_START + lane_idx;
                let turn_direction = match lane_idx {
                    0 => TurnDirection::Left,     // Lane 4 -> West 3
                    1 => TurnDirection::Straight, // Lane 5 -> North 5
                    2 => TurnDirection::Right,    // Lane 6 -> East 6
                    _ => unreachable!()
                };
                (lane, turn_direction)
            },
        };

        Vehicle::new(direction, lane, turn_direction)
    }

    fn get_target_lane(&self) -> i32 {
        match (self.direction, self.turn_direction) {
            // From East (Lanes 4,5,6)
            (Direction::East, TurnDirection::Left) => 3,     // Lane 4 -> North 4
            (Direction::East, TurnDirection::Straight) => 4, // Lane 5 -> East 5
            (Direction::East, TurnDirection::Right) => 0,    // Lane 6 -> South 1
            
            // From South (Lanes 1,2,3)
            (Direction::South, TurnDirection::Right) => 0,   // Lane 1 -> West 1
            (Direction::South, TurnDirection::Straight) => 1, // Lane 2 -> South 2
            (Direction::South, TurnDirection::Left) => 3,    // Lane 3 -> East 4
            
            // From West (Lanes 1,2,3)
            (Direction::West, TurnDirection::Right) => 5,    // Lane 1 -> North 6
            (Direction::West, TurnDirection::Straight) => 1, // Lane 2 -> West 2
            (Direction::West, TurnDirection::Left) => 2,     // Lane 3 -> South 3
            
            // From North (Lanes 4,5,6)
            (Direction::North, TurnDirection::Left) => 2,    // Lane 4 -> West 3
            (Direction::North, TurnDirection::Straight) => 4, // Lane 5 -> North 5
            (Direction::North, TurnDirection::Right) => 5,   // Lane 6 -> East 6
        }
    }

    fn get_turn_offset(&self) -> (i32, i32) {
        let offset = LANE_WIDTH;
        match (self.direction, self.turn_direction) {
            // From North
            (Direction::North, TurnDirection::Left) => (-offset, 0),
            (Direction::North, TurnDirection::Right) => (offset, 0),
            
            // From South
            (Direction::South, TurnDirection::Left) => (offset, 0),
            (Direction::South, TurnDirection::Right) => (-offset, 0),
            
            // From East
            (Direction::East, TurnDirection::Left) => (0, -offset),
            (Direction::East, TurnDirection::Right) => (0, offset),
            
            // From West
            (Direction::West, TurnDirection::Left) => (0, offset),
            (Direction::West, TurnDirection::Right) => (0, -offset),
            
            // Straight
            (_, TurnDirection::Straight) => (0, 0),
        }
    }

    fn get_lane_center(&self) -> (i32, i32) {
        let lane_offset = self.lane * LANE_WIDTH + (LANE_WIDTH / 2);
        let (offset_x, offset_y) = self.get_turn_offset();
        
        match self.direction {
            Direction::North => {
                let x = (WINDOW_WIDTH as i32 / 2) - (ROAD_VERTICAL_WIDTH as i32 / 2) + lane_offset;
                let y = (WINDOW_HEIGHT as i32 / 2) - (ROAD_VERTICAL_WIDTH as i32 / 2);
                (x + offset_x, y + offset_y)
            },
            Direction::South => {
                let x = (WINDOW_WIDTH as i32 / 2) + (ROAD_VERTICAL_WIDTH as i32 / 2) - lane_offset - LANE_WIDTH;
                let y = (WINDOW_HEIGHT as i32 / 2) + (ROAD_VERTICAL_WIDTH as i32 / 2);
                (x + offset_x, y + offset_y)
            },
            Direction::East => {
                let x = (WINDOW_WIDTH as i32 / 2) + (ROAD_HORIZONTAL_WIDTH as i32 / 2);
                let y = (WINDOW_HEIGHT as i32 / 2) - (ROAD_HORIZONTAL_WIDTH as i32 / 2) + lane_offset;
                (x + offset_x, y + offset_y)
            },
            Direction::West => {
                let x = (WINDOW_WIDTH as i32 / 2) - (ROAD_HORIZONTAL_WIDTH as i32 / 2);
                let y = (WINDOW_HEIGHT as i32 / 2) + (ROAD_HORIZONTAL_WIDTH as i32 / 2) - lane_offset - LANE_WIDTH;
                (x + offset_x, y + offset_y)
            },
        }
    }

    fn should_turn(&self) -> bool {
        if self.turning {
            return false;
        }

        let (lane_center_x, lane_center_y) = self.get_lane_center();

        match self.direction {
            Direction::North => self.pos_y <= lane_center_y,
            Direction::South => self.pos_y >= lane_center_y,
            Direction::East => self.pos_x >= lane_center_x,
            Direction::West => self.pos_x <= lane_center_x,
        }
    }

    fn get_new_direction(&self) -> Direction {
        match (self.direction, self.turn_direction) {
            (Direction::North, TurnDirection::Left) => Direction::West,
            (Direction::North, TurnDirection::Right) => Direction::East,
            (Direction::South, TurnDirection::Left) => Direction::East,
            (Direction::South, TurnDirection::Right) => Direction::West,
            (Direction::East, TurnDirection::Left) => Direction::North,
            (Direction::East, TurnDirection::Right) => Direction::South,
            (Direction::West, TurnDirection::Left) => Direction::South,
            (Direction::West, TurnDirection::Right) => Direction::North,
            (dir, TurnDirection::Straight) => dir,
        }
    }

    pub fn move_vehicle(&mut self) {
        let speed = VEHICLE_SPEED;

        if self.should_turn() && !self.turning {
            self.turning = true;
            let (lane_center_x, lane_center_y) = self.get_lane_center();
            
            // Set position to lane center for the turn
            match self.direction {
                Direction::North => self.pos_y = lane_center_y,
                Direction::South => self.pos_y = lane_center_y,
                Direction::East => self.pos_x = lane_center_x,
                Direction::West => self.pos_x = lane_center_x,
            }
            
            // Change direction and assign new lane
            self.direction = self.get_new_direction();
            self.lane = self.get_target_lane();
        }

        // Move in current direction
        match self.direction {
            Direction::North => self.pos_y -= speed,
            Direction::South => self.pos_y += speed,
            Direction::East => self.pos_x += speed,
            Direction::West => self.pos_x -= speed,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let rect = Rect::new(
            self.pos_x - (VEHICLE_WIDTH as i32 / 2),
            self.pos_y - (VEHICLE_HEIGHT as i32 / 2),
            VEHICLE_WIDTH,
            VEHICLE_HEIGHT,
        );

        let color = match self.turn_direction {
            TurnDirection::Left => Color::RGB(255, 100, 100),    // Red
            TurnDirection::Straight => Color::RGB(100, 255, 100), // Green
            TurnDirection::Right => Color::RGB(100, 100, 255),   // Blue
        };

        canvas.set_draw_color(color);
        canvas.fill_rect(rect).unwrap_or_else(|e| {
            eprintln!("Error rendering vehicle: {}", e);
        });
    }
}
