use crate::dimensions::Dimensions;
use crate::position::Position;

use crate::BLOCK_COLUMN_COUNT;
use crate::BLOCK_DIMENSIONS;
use crate::BLOCK_OFFSET_LEFT;
use crate::BLOCK_OFFSET_TOP;
use crate::BLOCK_PADDING;
use crate::BLOCK_ROW_COUNT;

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

    pub fn generate_blocks() -> Vec<Vec<Block>> {
        let mut blocks = Vec::new();

        for c in 0..BLOCK_COLUMN_COUNT {
            let col = Vec::new();
            blocks.push(col);
            for r in 0..BLOCK_ROW_COUNT {
                let block = Block::new(
                    Position::new(
                        (c as u16 * (BLOCK_DIMENSIONS.0 + BLOCK_PADDING)) + BLOCK_OFFSET_LEFT,
                        (r as u16 * (BLOCK_DIMENSIONS.1 + BLOCK_PADDING)) + BLOCK_OFFSET_TOP,
                    ),
                    Dimensions::new(BLOCK_DIMENSIONS.0, BLOCK_DIMENSIONS.1),
                    1,
                );
                blocks[c].push(block);
            }
        }
        blocks
    }
}
