use crate::dimensions::Dimensions;
use crate::position::Position;

#[derive(Debug)]
pub struct Block {
    pub position: Position,
    pub dimensions: Dimensions,
    pub strength: u16,
}

impl Block {
    pub fn new(position: Position, dimensions: Dimensions, strength: u16) -> Block {
        Block {
            position,
            dimensions,
            strength,
        }
    }

    /// Damage the block, from a contact with the ball
    pub fn damage(&mut self, amount: u16) {
        if amount > self.strength {
            self.strength = 0
        } else {
            self.strength -= amount
        }
    }
}
