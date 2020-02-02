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

use block_breaker_game::ball::Ball;
use block_breaker_game::bar::Bar;
use block_breaker_game::block::Block;
use block_breaker_game::dimensions::Dimensions;
use block_breaker_game::position::Position;
use block_breaker_game::vector::Vector;

use block_breaker_game::BAR_DIMENSIONS;
use block_breaker_game::BAR_STEP;
use block_breaker_game::DESIRED_FPS;
use block_breaker_game::SCREEN_SIZE;

#[derive(Debug)]
pub struct KeyPressed {
    pub right: bool,
    pub left: bool,
}

impl KeyPressed {
    pub fn new() -> Self {
        KeyPressed {
            right: false,
            left: false,
        }
    }
}

struct MainState {
    bar: Bar,
    ball: Ball,
    key_pressed: KeyPressed,
    blocks: Vec<Vec<Block>>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let main_state = MainState {
            bar: Bar::new(),
            ball: Ball::new(),
            key_pressed: KeyPressed::new(),
            blocks: Block::generate_blocks(),
        };
        Ok(main_state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let ball = &mut self.ball;
            let bar = &mut self.bar;
            let key_pressed = &self.key_pressed;

            //Handle Bar movement
            if key_pressed.left && bar.position.x() > 0 {
                bar.position -= Position::new(BAR_STEP, 0)
            }
            if key_pressed.right && bar.position.x() + bar.dimensions.width() < SCREEN_SIZE.0 as u16
            {
                bar.position += Position::new(BAR_STEP, 0)
            }

            //Handle wall bounce
            ball.handle_wall_bounce();

            let bar_x_min = bar.position.x();
            let bar_x_max = &bar_x_min + bar.dimensions.width();

            if ball.position.y() > bar.position.y() - ball.radius {
                if ball.position.x() > bar_x_min && ball.position.x() < bar_x_max {
                    ball.velocity.negate_y();
                } else {
                    println!("Game Over");
                    ball.position = Position::new(200, 250);
                    //let _ = event::quit(ctx);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, ggez::graphics::BLACK);
        //let mb = graphics::MeshBuilder::new();
        let blocks = &self.blocks;

        for c in 0..blocks.len() {
            for r in 0..blocks[c].len() {
                let b = &blocks[c][r];
                let blockrect = graphics::Rect::new(
                    b.position.x() as f32,
                    b.position.y() as f32,
                    b.dimensions.width() as f32,
                    b.dimensions.height() as f32,
                );
                let blockx = graphics::Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    blockrect,
                    graphics::WHITE,
                )?;
                graphics::draw(ctx, &blockx, DrawParam::new())?;
            }
        }

        //mb.build(ctx)?;

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
            ball.radius as f32,
            1.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &game_ball, DrawParam::default())?;
        graphics::present(ctx)?;
        timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => self.key_pressed.left = true,
            KeyCode::Right => self.key_pressed.right = true,
            _ => (),
        }
    }
    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Left => self.key_pressed.left = false,
            KeyCode::Right => self.key_pressed.right = false,
            _ => (),
        }
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
