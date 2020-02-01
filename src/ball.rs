use crate::dimensions::Dimensions;
use crate::position::Position;
use crate::vector::Vector;

pub struct Ball {
    pub position: Position,
    pub velocity: Vector,
    pub power: u16,
    pub game_dimensions: Dimensions,
    pub radius: u16,
}

impl Ball {
    pub fn new(position: Position, game_dimensions: Dimensions, power: u16) -> Ball {
        Ball {
            position,
            velocity: Vector::new(1., 1.),
            power,
            game_dimensions,
            radius: 12,
        }
    }

    // Make the ball take a step
    // Handles wall collision
    pub fn tick(&mut self) {
        let new_position = &self.position + &self.velocity;
        //Hit Left or Right wall
        if new_position.x() <= self.radius
            || new_position.x() >= self.game_dimensions.width() - self.radius
        {
            self.velocity.negate_x();
        }
        //Hit Top or Bottom wall
        if new_position.y() <= self.radius
            || new_position.y() >= self.game_dimensions.height() - self.radius
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
