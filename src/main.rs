mod tools;
mod car;

extern crate nalgebra as na;

use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::event::{self, KeyCode, KeyMods};
use na::Point2;

use car::Car;

struct MainState {
    car: Car,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            car: Car::new(Point2::new(300.0, 200.0)),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        use ggez::timer;

        let dt= timer::duration_to_f64(timer::delta(ctx)) as f32;

        self.car.update(dt);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.car.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, repeat: bool) {
        match keycode {
            KeyCode::W => self.car.set_throttle(1.0),
            KeyCode::S => self.car.set_throttle(-1.0),
            KeyCode::A => self.car.set_steering(-1.0),
            KeyCode::D => self.car.set_steering(1.0),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::W | KeyCode::S => self.car.reset_throttle(),
            KeyCode::A | KeyCode::D => self.car.reset_steering(),
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Skrr", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}