use crate::dimensions::Dimensions;
use crate::position::Position;

use crate::BAR_DIMENSIONS;
use crate::SCREEN_SIZE;

#[derive(Debug)]
pub struct Bar {
    pub dimensions: Dimensions,
    pub position: Position,
    pub game_dimensions: Dimensions,
}

impl Bar {
    pub fn new() -> Bar {
        let position = Position::new((SCREEN_SIZE.0 - BAR_DIMENSIONS.0) / 2, 550);
        let dimensions = Dimensions::new(BAR_DIMENSIONS.0, BAR_DIMENSIONS.1);
        let game_dimensions = Dimensions::new(SCREEN_SIZE.0, SCREEN_SIZE.1);
        Bar {
            position,
            dimensions,
            game_dimensions,
        }
    }
}
