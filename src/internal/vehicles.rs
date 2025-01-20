use super::lanes::Direction;
use super::constants::*;
use rand::Rng;

pub struct Vehicle {
    pub pos_x: i32,
    pub pos_y: i32,
    pub direction: Direction,
}

impl Vehicle {
    pub fn new(pos_x: i32, pos_y: i32, direction: Direction) -> Self {
        Vehicle {
            pos_x,
            pos_y,
            direction,
        }
    }

    /// Generates a random vehicle from a random direction
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let directions = [Direction::North, Direction::South, Direction::East, Direction::West];
        let direction = &directions[rng.gen_range(0..4)];

        // Calculate position based on direction
        let (pos_x, pos_y) = match direction {
            Direction::North => {
                let x = WINDOW_WIDTH as i32 / 2 - (ROAD_VERTICAL_WIDTH as i32 / 4) + rng.gen_range(0..ROAD_VERTICAL_WIDTH as i32 / 2);
                let y = WINDOW_HEIGHT as i32;
                (x, y)
            },
            Direction::South => {
                let x = WINDOW_WIDTH as i32 / 2 - (ROAD_VERTICAL_WIDTH as i32 / 4) + rng.gen_range(0..ROAD_VERTICAL_WIDTH as i32 / 2);
                let y = 0;
                (x, y)
            },
            Direction::East => {
                let x = 0;
                let y = WINDOW_HEIGHT as i32 / 2 - (ROAD_HORIZONTAL_WIDTH as i32 / 4) + rng.gen_range(0..ROAD_HORIZONTAL_WIDTH as i32 / 2);
                (x, y)
            },
            Direction::West => {
                let x = WINDOW_WIDTH as i32;
                let y = WINDOW_HEIGHT as i32 / 2 - (ROAD_HORIZONTAL_WIDTH as i32 / 4) + rng.gen_range(0..ROAD_HORIZONTAL_WIDTH as i32 / 2);
                (x, y)
            },
        };

        Vehicle::new(pos_x, pos_y, direction.clone())
    }

    pub fn move_vehicle(&mut self) {
        match self.direction {
            Direction::North => self.pos_y -= 1,
            Direction::South => self.pos_y += 1,
            Direction::East => self.pos_x += 1,
            Direction::West => self.pos_x -= 1,
        }
    }
}
