extern crate piston;
extern crate piston_window;
extern crate quick_xml;
extern crate rand;
extern crate serde;

mod utils;

use piston_window::*;
use std::time::SystemTime;
use utils::game_window::GameWindow;
use utils::scene::Scene;
use utils::sprite_sheet::SpriteSheet;

const WIDTH: u32 = 192;
const HEIGHT: u32 = 192;
const BACKGROUND: [f32; 4] = [0.328, 0.266, 0.480, 1.0];

const PLAYER_WALK_FPS: u32 = 24;
const PLAYER_WALK_FRAMES: [&str; 4] = [
    "playerRed_walk1.png",
    "playerRed_walk2.png",
    "playerRed_walk3.png",
    "playerRed_walk2.png",
];
const SCREEN_MOVE_PER_SECOND: f64 = 192.0;

#[derive(Clone)]
pub struct SpriteAnimationApp {
    width: u32,
    height: u32,
    sprite_sheet: SpriteSheet,
    player_size: [f64; 2],
    player_frame: u32,
    last_update: SystemTime,
    last_frame: SystemTime,
    plant_x: f64,
}

impl SpriteAnimationApp {
    pub fn new(width: u32, height: u32, sprite_sheet: SpriteSheet) -> SpriteAnimationApp {
        SpriteAnimationApp {
            width,
            height,
            sprite_sheet,
            player_size: [39.0, 48.0],
            player_frame: 0,
            last_update: SystemTime::now(),
            last_frame: SystemTime::now(),
            plant_x: 106.0,
        }
    }

    fn update_player(&mut self) {
        let dt = SystemTime::now()
            .duration_since(self.last_frame)
            .unwrap()
            .as_secs_f64();
        if dt >= 1.0 / PLAYER_WALK_FPS as f64 {
            self.player_frame = (self.player_frame + 1) % PLAYER_WALK_FRAMES.len() as u32;
            self.last_frame = SystemTime::now();
        }
    }
}

impl Scene for SpriteAnimationApp {
    fn render(self, c: Context, g: &mut G2d) {
        let h = self.height as f64;

        clear(BACKGROUND, g);
        self.sprite_sheet
            .render_sprite(&"tileYellow_06.png".to_string(), [0.0, h - 64.0], c, g);
        self.sprite_sheet.render_sprite(
            &"plantGreen_3.png".to_string(),
            [self.plant_x, h - 95.0],
            c,
            g,
        );
        self.sprite_sheet
            .render_sprite(&"tileYellow_06.png".to_string(), [64.0, h - 64.0], c, g);
        self.sprite_sheet
            .render_sprite(&"tileYellow_06.png".to_string(), [128.0, h - 64.0], c, g);
        self.sprite_sheet.render_sprite(
            &PLAYER_WALK_FRAMES[self.player_frame as usize].to_string(),
            [
                64.0 + (64.0 - self.player_size[0]) / 2.0,
                h - 64.0 - self.player_size[1],
            ],
            c,
            g,
        );
    }

    fn update(&mut self) {
        let dt = SystemTime::now()
            .duration_since(self.last_update)
            .unwrap()
            .as_secs_f64();

        self.update_player();
        self.plant_x -= SCREEN_MOVE_PER_SECOND * dt;
        if self.plant_x <= -44.0 {
            self.plant_x = self.width as f64;
        }

        self.last_update = SystemTime::now();
    }

    fn on_resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("sprite-sheet", [WIDTH, HEIGHT])
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

    let app = SpriteAnimationApp::new(WIDTH, HEIGHT, sheet);
    let mut game_window = GameWindow::new(window, app);

    game_window.game_loop();
}
