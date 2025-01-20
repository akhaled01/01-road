pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

// Road widths adjusted to be divisible by 6 (number of lanes)
pub const ROAD_HORIZONTAL_WIDTH: u32 = 498;  // 83 pixels per lane
pub const ROAD_VERTICAL_WIDTH: u32 = 498;    // 83 pixels per lane

// Intersection boundaries
pub const INTERSECTION_BOUNDARY_NORTH: u32 = (WINDOW_HEIGHT / 2) - (ROAD_VERTICAL_WIDTH / 2);
pub const INTERSECTION_BOUNDARY_SOUTH: u32 = (WINDOW_HEIGHT / 2) + (ROAD_VERTICAL_WIDTH / 2);
pub const INTERSECTION_BOUNDARY_WEST: u32 = (WINDOW_WIDTH / 2) - (ROAD_HORIZONTAL_WIDTH / 2);
pub const INTERSECTION_BOUNDARY_EAST: u32 = (WINDOW_WIDTH / 2) + (ROAD_HORIZONTAL_WIDTH / 2);

// Lane organization (3 lanes per direction)
pub const LANES_PER_SIDE: i32 = 3;
pub const TOTAL_LANES: i32 = LANES_PER_SIDE * 2;

// North road (left to right: 0-5)
pub const NORTH_OUTBOUND_START: i32 = 0;     // Left 3 lanes (0,1,2)
pub const NORTH_INBOUND_START: i32 = 3;      // Right 3 lanes (3,4,5)

// South road (left to right: 0-5)
pub const SOUTH_INBOUND_START: i32 = 0;      // Left 3 lanes (0,1,2)
pub const SOUTH_OUTBOUND_START: i32 = 3;     // Right 3 lanes (3,4,5)

// East road (top to bottom: 0-5)
pub const EAST_OUTBOUND_START: i32 = 0;      // Top 3 lanes (0,1,2)
pub const EAST_INBOUND_START: i32 = 3;       // Bottom 3 lanes (3,4,5)

// West road (top to bottom: 0-5)
pub const WEST_INBOUND_START: i32 = 0;       // Top 3 lanes (0,1,2)
pub const WEST_OUTBOUND_START: i32 = 3;      // Bottom 3 lanes (3,4,5)

// Vehicle constants
pub const VEHICLE_WIDTH: u32 = 30;
pub const VEHICLE_HEIGHT: u32 = 20;
pub const VEHICLE_SPEED: i32 = 2;

// Lane width
pub const LANE_WIDTH: i32 = (ROAD_HORIZONTAL_WIDTH as i32) / TOTAL_LANES;
