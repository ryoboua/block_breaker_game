use crate::dimensions::Dimensions;
use crate::position::Position;
use crate::vector::Vector;

use crate::SCREEN_SIZE;

pub struct Ball {
    pub position: Position,
    pub velocity: Vector,
    pub power: u16,
    pub game_dimensions: Dimensions,
    pub radius: u16,
}

impl Ball {
    pub fn new() -> Ball {
        let game_dimensions = Dimensions::new(SCREEN_SIZE.0, SCREEN_SIZE.1);
        let position = Position::new(SCREEN_SIZE.0 / 2, SCREEN_SIZE.1 / 2);

        Ball {
            position,
            game_dimensions,
            velocity: Vector::new(1., -1.),
            power: 1,
            radius: 12,
        }
    }

    // Handles wall collision
    pub fn handle_wall_bounce(&mut self) {
        let new_position = &self.position + &self.velocity;
        //Hit Left or Right wall
        if new_position.x() <= self.radius
            || new_position.x() >= self.game_dimensions.width() - self.radius
        {
            self.velocity.negate_x();
        }
        //Hit Top or Bottom wall
        if new_position.y() <= self.radius
        //|| new_position.y() >= self.game_dimensions.height() - self.radius
        {
            self.velocity.negate_y();
        }
        self.position = new_position
    }

    /// Bounce the ball off in a normal
    pub fn bounce(&mut self, normal: Vector) {
        self.velocity -= 2. * self.velocity.dot(&normal) * normal;
        self.velocity.normalise()
    }
}
