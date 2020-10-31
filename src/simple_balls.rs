extern crate piston;
extern crate piston_window;
extern crate quick_xml;
extern crate rand;
extern crate serde;

mod geometry;
mod utils;

use geometry::ball::{Ball, BallSettings, BallUpdateArgs};
use geometry::vector2d::Vector2d;
use geometry::velocity::Velocity;
use piston_window::*;
use std::time::SystemTime;
use utils::entity::{Renderable, Updatable};
use utils::game_window::GameWindow;
use utils::scene::Scene;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const CORNFLOWER_BLUE: [f32; 4] = [0.392, 0.584, 0.929, 1.0];

#[derive(Clone)]
pub struct SimpleBalls {
    balls: Vec<Ball>,
    width: u32,
    height: u32,
    last_update: SystemTime,
}

impl SimpleBalls {
    pub fn new(width: u32, height: u32) -> SimpleBalls {
        SimpleBalls {
            balls: (0..100)
                .map(|id| {
                    BallSettings::new()
                        .set_max_pos(WIDTH as f64, HEIGHT as f64)
                        .build(id)
                })
                .collect(),
            width,
            height,
            last_update: SystemTime::now(),
        }
    }
}

impl Scene for SimpleBalls {
    fn render(self, c: Context, g: &mut G2d) {
        clear(CORNFLOWER_BLUE, g);
        self.balls.iter().for_each(|b| b.render(c, g));
    }

    fn update(&mut self) {
        let duration = SystemTime::now().duration_since(self.last_update).unwrap();
        let w = self.width;
        let h = self.height;
        let dt = duration.as_secs_f64();
        let update_args = BallUpdateArgs {
            dt,
            width: w,
            height: h,
            balls: None,
        };
        self.balls
            .iter_mut()
            .for_each(|b| b.update(update_args.clone()));
        self.last_update = SystemTime::now();
    }

    fn on_resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

fn main() {
    let window = WindowSettings::new("simple-balls", [WIDTH, HEIGHT])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let app = SimpleBalls::new(WIDTH, HEIGHT);

    let mut game_window = GameWindow::new(window, app);
    game_window.game_loop();
}
