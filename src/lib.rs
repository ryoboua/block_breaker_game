pub mod ball;
pub mod bar;
pub mod block;
pub mod dimensions;
pub mod position;
pub mod vector;

//Game constants
pub const SCREEN_SIZE: (u16, u16) = (800, 600);
pub const BAR_DIMENSIONS: (u16, u16) = (100, 25);
pub const DESIRED_FPS: u32 = 400;
pub const BAR_STEP: u16 = 1;

pub const BLOCK_DIMENSIONS: (u16, u16) = (75, 25);
pub const BLOCK_ROW_COUNT: usize = 5;
pub const BLOCK_COLUMN_COUNT: usize = 9;
pub const BLOCK_OFFSET_TOP: u16 = 10;
pub const BLOCK_OFFSET_LEFT: u16 = 10;
pub const BLOCK_PADDING: u16 = 10;
