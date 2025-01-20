use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use super::lanes::Direction;
use super::constants::*;
use rand::Rng;
use rand::seq::SliceRandom;

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
        let lane_offset = lane * LANE_WIDTH + (LANE_WIDTH / 2);
        
        match direction {
            Direction::North => {
                let x = (WINDOW_WIDTH as i32 / 2) - (ROAD_VERTICAL_WIDTH as i32 / 2) + lane_offset;
                let y = WINDOW_HEIGHT as i32;
                (x, y)
            },
            Direction::South => {
                let x = (WINDOW_WIDTH as i32 / 2) + (ROAD_VERTICAL_WIDTH as i32 / 2) - lane_offset - LANE_WIDTH;
                let y = 0;
                (x, y)
            },
            Direction::East => {
                let x = 0;
                let y = (WINDOW_HEIGHT as i32 / 2) - (ROAD_HORIZONTAL_WIDTH as i32 / 2) + lane_offset;
                (x, y)
            },
            Direction::West => {
                let x = WINDOW_WIDTH as i32;
                let y = (WINDOW_HEIGHT as i32 / 2) + (ROAD_HORIZONTAL_WIDTH as i32 / 2) - lane_offset - LANE_WIDTH;
                (x, y)
            },
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
        let direction = directions[rng.gen_range(0..4)];
        
        // Determine valid lanes based on direction
        let (lane_start, lane_count) = match direction {
            // North & East: Lanes 1-3 are outbound (0-2)
            Direction::North | Direction::East => (0, 3),
            // West & South: Lanes 4-6 are outbound (3-5)
            Direction::West | Direction::South => (3, 3),
        };
        
        let lane = lane_start + rng.gen_range(0..lane_count);
        
        // Assign turn direction based on lane position
        let turn_direction = match direction {
            // For North/East, only lane 0 can turn right
            Direction::North | Direction::East => {
                if lane == 0 {
                    // First lane can do any turn
                    let turns = [TurnDirection::Left, TurnDirection::Straight, TurnDirection::Right];
                    *turns.choose(&mut rng).unwrap()
                } else if lane == 2 {
                    // Last lane can only turn left or go straight
                    if rng.gen_bool(0.5) {
                        TurnDirection::Left
                    } else {
                        TurnDirection::Straight
                    }
                } else {
                    // Middle lane goes straight
                    TurnDirection::Straight
                }
            },
            // For West/South, only lane 5 can turn right
            Direction::West | Direction::South => {
                if lane == 5 {
                    // Last lane can do any turn
                    let turns = [TurnDirection::Left, TurnDirection::Straight, TurnDirection::Right];
                    *turns.choose(&mut rng).unwrap()
                } else if lane == 3 {
                    // First lane can only turn left or go straight
                    if rng.gen_bool(0.5) {
                        TurnDirection::Left
                    } else {
                        TurnDirection::Straight
                    }
                } else {
                    // Middle lane goes straight
                    TurnDirection::Straight
                }
            }
        };

        Vehicle::new(direction, lane, turn_direction)
    }

    fn get_target_lane(&self) -> i32 {
        match (self.direction, self.turn_direction) {
            // From North (outbound lanes 0,1,2)
            (Direction::North, TurnDirection::Right) => 0,    // To East's first lane
            (Direction::North, TurnDirection::Straight) => 1, // To South's second outbound lane
            (Direction::North, TurnDirection::Left) => 5,     // To West's last lane
            
            // From South (outbound lanes 3,4,5)
            (Direction::South, TurnDirection::Right) => 5,    // To West's last lane
            (Direction::South, TurnDirection::Straight) => 4, // To North's second outbound lane
            (Direction::South, TurnDirection::Left) => 0,     // To East's first lane
            
            // From East (outbound lanes 0,1,2)
            (Direction::East, TurnDirection::Right) => 0,     // To South's first lane
            (Direction::East, TurnDirection::Straight) => 1,  // To West's second outbound lane
            (Direction::East, TurnDirection::Left) => 5,      // To North's last lane
            
            // From West (outbound lanes 3,4,5)
            (Direction::West, TurnDirection::Right) => 5,     // To North's last lane
            (Direction::West, TurnDirection::Straight) => 4,  // To East's second outbound lane
            (Direction::West, TurnDirection::Left) => 0,      // To South's first lane
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
