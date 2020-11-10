extern crate piston;
extern crate piston_window;
extern crate quick_xml;
extern crate rand;
extern crate serde;

mod platformer;
mod utils;

use piston_window::*;
use platformer::game_world::{GameWorld, GameWorldUpdateArgs};
use std::time::SystemTime;
use utils::entity::*;
use utils::game_window::GameWindow;
use utils::input::InputHandler;
use utils::scene::Scene;
use utils::sprite_sheet::SpriteSheet;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 448;

#[derive(Clone)]
pub struct PlatformerApp {
    world: GameWorld,
    last_update: SystemTime,
}

impl PlatformerApp {
    fn new(width: u32, height: u32, sprite_sheet: SpriteSheet) -> PlatformerApp {
        PlatformerApp {
            world: GameWorld::new(width, height, sprite_sheet),
            last_update: SystemTime::now(),
        }
    }
}

impl Scene for PlatformerApp {
    fn render(self, c: Context, g: &mut G2d) {
        self.world.render(c, g);
    }

    fn update(&mut self) {
        let dt = SystemTime::now()
            .duration_since(self.last_update)
            .unwrap()
            .as_secs_f64();
        self.world.update(GameWorldUpdateArgs { dt });
        self.last_update = SystemTime::now();
    }

    fn on_resize(&mut self, new_width: u32, new_height: u32) {
        self.world.width = new_width;
        self.world.height = new_height;
    }
}

impl InputHandler for PlatformerApp {
    fn on_button_event(&mut self, args: ButtonArgs) {
        self.world.input.on_key_event(args);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("platformer", [WIDTH, HEIGHT])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // assets from Kenney, find out more at https://kenney.nl/
    let sheet = SpriteSheet::new(
        "assets",
        "sprites.xml",
        &mut window.create_texture_context(),
    );

    let app = PlatformerApp::new(WIDTH, HEIGHT, sheet);
    let mut game_window = GameWindow::new(window, app);

    game_window.game_loop();
}
