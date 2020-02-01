use crate::dimensions::Dimensions;
use crate::position::Position;

#[derive(Debug)]
pub struct Bar {
    pub dimensions: Dimensions,
    pub position: Position,
    pub game_dimensions: Dimensions
}

impl Bar {
    pub fn new(position: Position, dimensions: Dimensions, game_dimensions: Dimensions) -> Bar {
        Bar {
            position,
            dimensions,
            game_dimensions,
        }
    }
}