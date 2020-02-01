use crate::dimensions::Dimensions;
use crate::position::Position;
use crate::vector::Vector;

pub struct Ball {
    pub position: Position,
    pub velocity: Vector,
    pub power: u16,
    pub game_dimensions: Dimensions,
}

impl Ball {
    pub fn new(position: Position, game_dimensions: Dimensions, power: u16) -> Ball {
        Ball {
            position,
            velocity: Vector::new(-1., 1.),
            power,
            game_dimensions,
        }
    }

    /// Make the ball take a step
    pub fn tick(&mut self) {
        let mut new_position = &self.position + &self.velocity;
        //Hit Left wall
        if new_position.x() <= 0 {
            self.bounce(Vector::new(1.0, 0.));
            //new_position = Position::new(new_position.x(), self.game_dimensions.height())
        }
        //Hit Right wall
        if new_position.x() >= self.game_dimensions.width() {
            self.bounce(Vector::new(-1.0, 0.));
            //new_position = Position::new(self.game_dimensions.width() - 1, new_position.y())
        }
        //Hit Top wall
        if new_position.y() <= 0 {
            self.bounce(Vector::new(0., 1.0));
            //new_position = Position::new(new_position.x(), self.game_dimensions.height())
        }
        //Hit Bottom wall
        if new_position.y() >= self.game_dimensions.height() {
            self.bounce(Vector::new(0., -1.0));
            //new_position = Position::new(new_position.x(), self.game_dimensions.height())
        }
        // if new_position.y() >= self.game_dimensions.height() - 1 {
        //     self.velocity = Vector::new(0., 0.)
        // }
        self.position = new_position
    }

    /// Bounce the ball off in a normal
    pub fn bounce(&mut self, normal: Vector) {
        self.velocity -= 2. * self.velocity.dot(&normal) * normal;
        self.velocity.normalise()
    }
}
