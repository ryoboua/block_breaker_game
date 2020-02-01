#![allow(unused_variables)]
#![allow(unused_imports)]

use ggez;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::StrokeOptions;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};

mod ball;
mod bar;
mod dimensions;
mod position;
mod vector;

use ball::Ball;
use bar::Bar;
use dimensions::Dimensions;
use position::Position;
use vector::Vector;

const SCREEN_SIZE: (u16, u16) = (800, 500);
const BAR_DIMENSIONS: (u16, u16) = (100, 25);

struct MainState {
    bar: Bar,
    ball: Ball,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let game_dimensions = Dimensions::new(SCREEN_SIZE.0, SCREEN_SIZE.1);

        let bar_dismensions = Dimensions::new(BAR_DIMENSIONS.0, BAR_DIMENSIONS.1);
        let initial_bar_position = Position::new(50, 450);
        let bar = Bar::new(
            initial_bar_position,
            bar_dismensions,
            game_dimensions.clone(),
        );

        let initial_ball_position = Position::new(200, 250);
        let initial_ball_power = 10;

        let ball = Ball::new(
            initial_ball_position,
            game_dimensions.clone(),
            initial_ball_power,
        );
        let s = MainState { bar, ball };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 80;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.ball.tick();
            let ball = &mut self.ball;
            let bar = &self.bar;
            let data = bar.position.y();
            let bar_x_min = bar.position.x();
            let bar_x_max = &bar_x_min + self.bar.dimensions.width();
            if ball.position.y() > bar.position.y() {
                if ball.position.x() > bar_x_min && ball.position.x() < bar_x_max {
                    ball.bounce(Vector::new(0., -2.0));
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, ggez::graphics::BLACK);

        let bar = &self.bar;
        let rect = graphics::Rect::new(
            bar.position.x() as f32,
            bar.position.y() as f32,
            bar.dimensions.width() as f32,
            bar.dimensions.height() as f32,
        );
        let bar =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;

        graphics::draw(ctx, &bar, DrawParam::default())?;
        let ball = &self.ball;
        let ball_point = na::Point2::new(ball.position.x() as f32, ball.position.y() as f32);
        let game_ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            ball_point,
            12.0,
            1.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &game_ball, DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }

    /// key_down_event gets fired when a key gets pressed.
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        const SHIFTER: u16 = 50;
        match keycode {
            KeyCode::Left => {
                if self.bar.position.x() > 0 {
                    self.bar.position -= Position::new(SHIFTER, 0)
                }
            }
            KeyCode::Right => {
                if self.bar.position.x() + self.bar.dimensions.width() < SCREEN_SIZE.0 as u16 {
                    self.bar.position += Position::new(SHIFTER, 0)
                }
            }
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        println!("Ball X position {:#?}", self.ball.position.x());
        println!("Bar X position {:#?}", self.bar.position.x());
        println!(
            "Bar X position + width {:#?}",
            self.bar.position.x() + self.bar.dimensions.width()
        );
    }
}
pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Block Breaker", "Reggie")
        .window_setup(ggez::conf::WindowSetup::default().title("Block Breaker!"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        );
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
