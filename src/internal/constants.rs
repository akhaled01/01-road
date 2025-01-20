pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

// Road widths adjusted to be divisible by 6 (number of lanes)
pub const ROAD_HORIZONTAL_WIDTH: u32 = 498;  // 83 pixels per lane
pub const ROAD_VERTICAL_WIDTH: u32 = 498;    // 83 pixels per lane

pub const INTERSECTION_BOUNDARY_NOTRTH: u32 = (WINDOW_HEIGHT / 2) - (ROAD_VERTICAL_WIDTH / 2);
pub const INTERSECTION_BOUNDARY_SOUTH: u32 = (WINDOW_HEIGHT / 2) + (ROAD_VERTICAL_WIDTH / 2);
pub const INTERSECTION_BOUNDARY_WEST: u32 = (WINDOW_WIDTH / 2) - (ROAD_HORIZONTAL_WIDTH / 2);
pub const INTERSECTION_BOUNDARY_EAST: u32 = (WINDOW_WIDTH / 2) + (ROAD_HORIZONTAL_WIDTH / 2);
